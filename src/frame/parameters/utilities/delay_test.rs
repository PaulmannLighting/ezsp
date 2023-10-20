use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x009D;

/// Used to test that UART flow control is working correctly.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    delay: u16,
}

impl Command {
    #[must_use]
    pub const fn new(delay: u16) -> Self {
        Self { delay }
    }

    #[must_use]
    pub const fn delay(&self) -> u16 {
        self.delay
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.delay.to_be_bytes().into_iter()
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            delay: src.read_num_be()?,
        })
    }
}

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
    fn read_from<R>(_: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {})
    }
}
