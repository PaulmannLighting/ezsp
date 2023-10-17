use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x0026;

/// Returns the EUI64 ID of the local node.
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

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(_: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    eui64: u64,
}

impl Response {
    #[must_use]
    pub const fn new(eui64: u64) -> Self {
        Self { eui64 }
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        self.eui64.to_be_bytes().into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer = [0; 8];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            eui64: u64::from_be_bytes(buffer),
        })
    }
}
