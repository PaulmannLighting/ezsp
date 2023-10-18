use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Once};

pub const ID: u16 = 0x0057;

/// Triggers a pan id update message.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    new_pan: u16,
}

impl Command {
    #[must_use]
    pub const fn new(new_pan: u16) -> Self {
        Self { new_pan }
    }

    #[must_use]
    pub const fn new_pan(&self) -> u16 {
        self.new_pan
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.new_pan.to_be_bytes().into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            new_pan: u16::from_be_bytes(buffer),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: bool,
}

impl Response {
    #[must_use]
    pub const fn new(status: bool) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> bool {
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

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: status != 0,
        })
    }
}
