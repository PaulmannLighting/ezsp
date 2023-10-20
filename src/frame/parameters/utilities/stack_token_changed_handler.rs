use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;

pub const ID: u16 = 0x000D;

/// A callback invoked to inform the application that a stack token has changed.
#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    token_address: u16,
}

impl Response {
    #[must_use]
    pub const fn new(token_address: u16) -> Self {
        Self { token_address }
    }

    #[must_use]
    pub const fn token_address(&self) -> u16 {
        self.token_address
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.token_address.to_be_bytes().into_iter()
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            token_address: src.read_num_be()?,
        })
    }
}
