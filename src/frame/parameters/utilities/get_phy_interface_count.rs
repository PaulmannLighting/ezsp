use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x00FC;

/// Returns the 16-bit node ID of the local node.
#[derive(Debug, Eq, PartialEq)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        empty()
    }
}

impl Readable for Command {
    fn try_read<R>(_: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    interface_count: u8,
}

impl Response {
    #[must_use]
    pub const fn new(interface_count: u8) -> Self {
        Self { interface_count }
    }

    #[must_use]
    pub const fn interface_count(&self) -> u8 {
        self.interface_count
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        self.interface_count.to_le_bytes().into_iter()
    }
}

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        Ok(Self {
            interface_count: src.read_num_le()?,
        })
    }
}
