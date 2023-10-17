use crate::frame::Parameters;
use crate::mfg_token;
use crate::status::Status;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;
use std::{array, vec};

pub const ID: u16 = 0x000C;

/// Sets a manufacturing token in the Customer Information Block (CIB)
/// area of the NCP if that token currently unset (fully erased).
///
/// Cannot be used with EZSP_STACK_CAL_DATA, EZSP_STACK_CAL_FILTER,
/// EZSP_MFG_ASH_CONFIG, or EZSP_MFG_CBKE_DATA token.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    token_id: mfg_token::Id,
    token_data_length: u8,
    token_data: Arc<[u8]>,
}

impl Command {
    /// Creates a new [`Command`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `token_data` exceeds the bounds of an u8.
    pub fn new(token_id: mfg_token::Id, token_data: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
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

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(2 + self.token_data.len());
        parameters.push(self.token_id.into());
        parameters.push(self.token_data_length);
        parameters.extend_from_slice(&self.token_data);
        parameters.into_iter()
    }
}
impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [token_id, token_data_length] = [0; 2];
        src.read_exact(&mut buffer)?;
        let mut token_data = vec![0; token_data_length.into()];
        src.read_exact(&mut token_data)?;
        Ok(Self {
            token_id: mfg_token::Id::try_from(token_id)?,
            token_data_length,
            token_data: token_data.into(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = array::IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.status.into()].into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(status)?,
        })
    }
}
