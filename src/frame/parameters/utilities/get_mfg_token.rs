use crate::ezsp::mfg_token::Id;
use crate::read_write::Readable;
use crate::types::ByteSizedVec;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};
use std::vec::IntoIter;

pub const ID: u16 = 0x000B;

/// Retrieves a manufacturing token from the Flash Information Area of the NCP
/// (except for [`Id::Stack`]`(`[`CalData`][`crate::mfg_token::stack::Stack::CalData`]`)`
/// which is managed by the stack).
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    token_id: Id,
}

impl Command {
    #[must_use]
    pub const fn new(token_id: Id) -> Self {
        Self { token_id }
    }

    #[must_use]
    pub const fn token_id(&self) -> Id {
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

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let token_id: u8 = src.read_num_le()?;
        Ok(Self {
            token_id: token_id.try_into()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    token_data: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(token_data: ByteSizedVec<u8>) -> Self {
        Self { token_data }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn token_data_length(&self) -> u8 {
        self.token_data.len() as u8
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
        parameters.push(self.token_data_length());
        parameters.extend_from_slice(&self.token_data);
        parameters.into_iter()
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let token_data_length: u8 = src.read_num_le()?;
        Ok(Self {
            token_data: unsafe { src.read_heapless_vec_exact(token_data_length as usize)? },
        })
    }
}
