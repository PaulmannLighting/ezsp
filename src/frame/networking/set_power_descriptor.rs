use crate::frame::Parameters;
use crate::util::ReadExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x0016;

/// Sets the power descriptor to the specified value.
///
/// The power descriptor is a dynamic value.
/// Therefore, you should call this function whenever the value changes.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    descriptor: u16,
}

impl Command {
    #[must_use]
    pub const fn new(descriptor: u16) -> Self {
        Self { descriptor }
    }

    #[must_use]
    pub const fn descriptor(&self) -> u16 {
        self.descriptor
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.descriptor.to_be_bytes().into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            descriptor: src.read_num_be()?,
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

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(_: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {})
    }
}
