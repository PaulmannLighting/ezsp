use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::mfg_token;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x000B;

/// Retrieves a manufacturing token from the Flash Information Area of the NCP
/// (except for EZSP_STACK_CAL_DATA which is managed by the stack).
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    token_id: mfg_token::Id,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, token_id: mfg_token::Id) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            token_id,
        }
    }

    #[must_use]
    pub const fn token_id(&self) -> mfg_token::Id {
        self.token_id
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.token_id.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [token_id]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            token_id: mfg_token::Id::try_from(token_id)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    token_data_length: u8,
    token_data: Arc<[u8]>,
}

impl Response {
    /// Creates a new [`Response`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `token_data` exceeds the bounds of an u8.
    pub fn new(
        sequence: u8,
        control: Control,
        token_data: Arc<[u8]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
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

impl Frame<ID> for Response {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(1 + self.token_data.len());
        parameters.push(self.token_data_length);
        parameters.extend_from_slice(&self.token_data);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [token_data_length]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        let mut token_data = Vec::with_capacity(token_data_length.into());
        src.read_exact(&mut token_data)?;
        Ok(Self {
            header,
            token_data_length,
            token_data: token_data.into(),
        })
    }
}
