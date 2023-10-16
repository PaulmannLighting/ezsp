use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::mfg_token;
use crate::status::Status;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x000C;

/// Sets a manufacturing token in the Customer Information Block (CIB)
/// area of the NCP if that token currently unset (fully erased).
///
/// Cannot be used with EZSP_STACK_CAL_DATA, EZSP_STACK_CAL_FILTER,
/// EZSP_MFG_ASH_CONFIG, or EZSP_MFG_CBKE_DATA token.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    token_id: mfg_token::Id,
    token_data_length: u8,
    token_data: Arc<[u8]>,
}

impl Command {
    /// Creates a new [`Command`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `token_data` exceeds the bounds of an u8.
    pub fn new(
        sequence: u8,
        control: Control,
        token_id: mfg_token::Id,
        token_data: Arc<[u8]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
            token_id,
            token_data_length: token_data.len().try_into()?,
            token_data,
        })
    }

    #[must_use]
    pub const fn token_id(&self) -> mfg_token::Id {
        self.token_id
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

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(2 + self.token_data.len());
        parameters.push(self.token_id.into());
        parameters.push(self.token_data_length);
        parameters.extend_from_slice(&self.token_data);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [token_id, token_data_length] = [0; 2];
        src.read_exact(&mut buffer)?;
        let mut token_data = vec![0; token_data_length.into()];
        src.read_exact(&mut token_data)?;
        Ok(Self {
            header,
            token_id: mfg_token::Id::try_from(token_id)?,
            token_data_length,
            token_data: token_data.into(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, status: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            status: Status::try_from(status)?,
        })
    }
}
