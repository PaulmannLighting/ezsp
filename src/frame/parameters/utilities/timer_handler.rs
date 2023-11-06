use crate::read_write::Readable;
use std::io::Read;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x000F;

/// A callback from the timer.
#[derive(Debug, Eq, PartialEq)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        empty()
    }
}

impl Readable for Response {
    fn try_read<R>(_: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        Ok(Self {})
    }
}
