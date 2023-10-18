use crate::frame::Parameters;
use crate::mfg_token;
use std::io::Read;
use std::iter::{once, Once};
use std::num::TryFromIntError;
use std::sync::Arc;
use std::vec::IntoIter;

pub const ID: u16 = 0x000B;

/// Retrieves a manufacturing token from the Flash Information Area of the NCP
/// (except for EZSP_STACK_CAL_DATA which is managed by the stack).
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    token_id: mfg_token::Id,
}

impl Command {
    #[must_use]
    pub const fn new(token_id: mfg_token::Id) -> Self {
        Self { token_id }
    }

    #[must_use]
    pub const fn token_id(&self) -> mfg_token::Id {
        self.token_id
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.token_id.into())
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [token_id] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            token_id: mfg_token::Id::try_from(token_id)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    token_data_length: u8,
    token_data: Arc<[u8]>,
}

impl Response {
    /// Creates a new [`Response`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `token_data` exceeds the bounds of an u8.
    pub fn new(token_data: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
            token_data_length: token_data.len().try_into()?,
            token_data,
        })
    }

    #[must_use]
    pub const fn token_data_length(&self) -> u8 {
        self.token_data_length
    }

    #[must_use]
    pub fn token_data(&self) -> &[u8] {
        &self.token_data
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(1 + self.token_data.len());
        parameters.push(self.token_data_length);
        parameters.extend_from_slice(&self.token_data);
        parameters.into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [token_data_length] = [0; 1];
        src.read_exact(&mut buffer)?;
        let mut token_data = vec![0; token_data_length.into()];
        src.read_exact(&mut token_data)?;
        Ok(Self {
            token_data_length,
            token_data: token_data.into(),
        })
    }
}
