//! Ember token types.

use std::array::IntoIter;
use std::fmt::{self, Debug, Formatter};
use std::iter::{Chain, Take};

use le_stream::{FromLeStream, ToLeStream};

use crate::ValueError;

/// Maximum supported token payload length in bytes.
pub const MAX_TOKEN_DATA_SIZE: usize = MAX_TOKEN_DATA_LENGTH as usize;

const MAX_TOKEN_DATA_LENGTH: u32 = 64;

const SIZE_LENGTH: usize = size_of::<u32>();
const EMPTY_DATA: [u8; MAX_TOKEN_DATA_SIZE] = [0; MAX_TOKEN_DATA_SIZE];

/// Bounded token data, encoded as a four-byte length followed by that many bytes.
///
/// Debug output is redacted because tokens can contain security keys. This type
/// does not zeroize memory. The backing array's unused bytes are always zero.
#[derive(Clone, Eq, PartialEq)]
pub struct Data {
    size: u32,
    data: [u8; MAX_TOKEN_DATA_SIZE],
}

impl Data {
    /// Create token data, discarding bytes beyond the declared payload length.
    ///
    /// # Panics
    /// Panics if `size` exceeds [`MAX_TOKEN_DATA_SIZE`]. Use [`Self::try_new`]
    /// when constructing data from an untrusted length.
    #[must_use]
    pub const fn new(size: u32, data: [u8; MAX_TOKEN_DATA_SIZE]) -> Self {
        assert!(size <= MAX_TOKEN_DATA_LENGTH, "token payload is too large");
        let mut payload = EMPTY_DATA;
        let mut index = 0;
        while index < size as usize {
            payload[index] = data[index];
            index += 1;
        }
        Self {
            size,
            data: payload,
        }
    }

    /// Create token data with a checked length.
    ///
    /// # Errors
    /// Returns [`ValueError::InvalidTokenSize`] if the payload is too large.
    pub const fn try_new(size: u32, data: [u8; MAX_TOKEN_DATA_SIZE]) -> Result<Self, ValueError> {
        if size > MAX_TOKEN_DATA_LENGTH {
            return Err(ValueError::InvalidTokenSize(size));
        }
        Ok(Self::new(size, data))
    }

    /// Return token payload size in bytes.
    #[must_use]
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Return the zero-padded backing array. Only [`Self::as_slice`] is transmitted.
    #[must_use]
    pub const fn data(&self) -> &[u8; MAX_TOKEN_DATA_SIZE] {
        &self.data
    }

    /// Explicitly access the secret-bearing payload, excluding unused bytes.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.size as usize]
    }
}

impl Debug for Data {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Data")
            .field("size", &self.size)
            .field("data", &"[REDACTED]")
            .finish()
    }
}

impl FromLeStream for Data {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let size = u32::from_le_stream(&mut bytes)?;
        if size > MAX_TOKEN_DATA_LENGTH {
            return None;
        }
        let mut data = EMPTY_DATA;
        for byte in &mut data[..size as usize] {
            *byte = bytes.next()?;
        }
        Some(Self { size, data })
    }
}

impl ToLeStream for Data {
    type Iter = Chain<IntoIter<u8, SIZE_LENGTH>, Take<IntoIter<u8, MAX_TOKEN_DATA_SIZE>>>;

    fn to_le_stream(self) -> Self::Iter {
        self.size
            .to_le_bytes()
            .into_iter()
            .chain(self.data.into_iter().take(self.size as usize))
    }
}

/// Information of a token in the token table.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Info {
    nvm3_key: u32,
    is_cnt: bool,
    is_idx: bool,
    size: u8,
    array_size: u8,
}

impl Info {
    /// Create new token info.
    #[must_use]
    pub const fn new(nvm3_key: u32, is_cnt: bool, is_idx: bool, size: u8, array_size: u8) -> Self {
        Self {
            nvm3_key,
            is_cnt,
            is_idx,
            size,
            array_size,
        }
    }

    /// Return NVM3 key of the token.
    #[must_use]
    pub const fn nvm3_key(&self) -> u32 {
        self.nvm3_key
    }

    /// Return whether token is a counter type.
    #[must_use]
    pub const fn is_cnt(&self) -> bool {
        self.is_cnt
    }

    /// Return whether token is an indexed token.
    #[must_use]
    pub const fn is_idx(&self) -> bool {
        self.is_idx
    }

    /// Return size of the token.
    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    /// Return array size of the token.
    #[must_use]
    pub const fn array_size(&self) -> u8 {
        self.array_size
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};

    use super::{Data, MAX_TOKEN_DATA_SIZE};
    use crate::ValueError;
    use crate::parameters::token_interface::{get_token_data, set_token_data};

    const TOKEN: u32 = 0x1234_5678;
    const INDEX: u32 = 2;
    const SECRET: [u8; MAX_TOKEN_DATA_SIZE] = [0xAB; MAX_TOKEN_DATA_SIZE];

    #[test]
    fn encodes_only_declared_bytes_and_zeroes_unused_storage() {
        for size in 0..=MAX_TOKEN_DATA_SIZE {
            let data = Data::try_new(u32::try_from(size).unwrap(), SECRET).unwrap();
            let encoded: Vec<_> = data.clone().to_le_stream().collect();
            assert_eq!(
                &encoded[..size_of::<u32>()],
                &u32::try_from(size).unwrap().to_le_bytes()
            );
            assert_eq!(&encoded[size_of::<u32>()..], &SECRET[..size]);
            assert_eq!(data.as_slice(), &SECRET[..size]);
            assert!(data.data()[size..].iter().all(|byte| *byte == 0));
            assert_eq!(
                Data::from_le_stream_exact(encoded.into_iter()).unwrap(),
                data
            );
        }
    }

    #[test]
    fn rejects_invalid_lengths_and_truncated_payloads() {
        assert!(matches!(
            Data::try_new(65, SECRET),
            Err(ValueError::InvalidTokenSize(65))
        ));
        for size in [65, u32::MAX] {
            assert!(Data::from_le_stream(size.to_le_bytes().into_iter().chain(SECRET)).is_none());
        }
        assert!(Data::from_le_stream([1, 0, 0].into_iter()).is_none());
        assert!(Data::from_le_stream([2, 0, 0, 0, 0xAB].into_iter()).is_none());
    }

    #[test]
    fn decoding_leaves_following_fields_unconsumed() {
        let mut stream = [1, 0, 0, 0, 0xAB, 0xFF].into_iter();
        assert_eq!(
            Data::from_le_stream(&mut stream).unwrap().as_slice(),
            &[0xAB]
        );
        assert_eq!(stream.next(), Some(0xFF));
    }

    #[test]
    fn token_commands_use_variable_length_wire_payloads() {
        let command = set_token_data::Command::new(TOKEN, INDEX, Data::new(2, SECRET));
        assert_eq!(
            command.to_le_stream().collect::<Vec<_>>(),
            [0x78, 0x56, 0x34, 0x12, 2, 0, 0, 0, 2, 0, 0, 0, 0xAB, 0xAB]
        );
        let response =
            get_token_data::Response::from_le_stream_exact([0, 2, 0, 0, 0, 0xAB, 0xAB].into_iter())
                .unwrap();
        assert_eq!(Data::try_from(response).unwrap().as_slice(), &[0xAB, 0xAB]);
    }

    #[test]
    fn debug_output_does_not_expose_token_data() {
        assert_eq!(
            format!("{:?}", Data::new(2, SECRET)),
            "Data { size: 2, data: \"[REDACTED]\" }"
        );
    }
}
