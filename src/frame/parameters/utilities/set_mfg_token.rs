use crate::ember::Status;
use crate::mfg_token::Id;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};
use std::num::TryFromIntError;
use std::sync::Arc;
use std::vec::IntoIter;

pub const ID: u16 = 0x000C;

/// Sets a manufacturing token in the Customer Information Block (CIB)
/// area of the NCP if that token currently unset (fully erased).
///
/// Cannot be used with
/// [`Id::Stack`]`(`[`Stack::CalData`](crate::mfg_token::stack::Stack::CalData)`)`,
/// [`Id::Stack`]`(`[`Stack::CalFilter`](crate::mfg_token::stack::Stack::CalFilter)`)`,
/// [`Id::Mfg`]`(`[`Mfg::AshConfig`](crate::mfg_token::mfg::Mfg::AshConfig)`)`, or
/// [`Id::Mfg`]`(`[`Mfg::CbkeData`](crate::mfg_token::mfg::Mfg::CbkeData)`)` token.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    token_id: Id,
    token_data_length: u8,
    token_data: Arc<[u8]>,
}

impl Command {
    /// Creates a new [`Command`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `token_data` exceeds the bounds of an u8.
    pub fn new(token_id: Id, token_data: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
            token_id,
            token_data_length: token_data.len().try_into()?,
            token_data,
        })
    }

    #[must_use]
    pub const fn token_id(&self) -> Id {
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
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(2 + self.token_data.len());
        parameters.push(self.token_id.into());
        parameters.push(self.token_data_length);
        parameters.extend_from_slice(&self.token_data);
        parameters.into_iter()
    }
}
impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [token_id, token_data_length] = src.read_array_exact()?;
        let token_data = src.read_vec_exact(token_data_length.into())?;
        Ok(Self {
            token_id: token_id.try_into()?,
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
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_be()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
