use crate::ezsp_status::EzspStatus;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x0049;

/// Returns a pseudorandom number.
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
    status: EzspStatus,
    value: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus, value: u16) -> Self {
        Self { status, value }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }

    #[must_use]
    pub fn succeeded(&self) -> bool {
        self.status == EzspStatus::Success
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        let [value_low, value_high] = self.value.to_be_bytes();
        [self.status.into(), value_low, value_high].into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status, value @ ..] = [0; 3];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: EzspStatus::try_from(status)?,
            value: u16::from_be_bytes(value),
        })
    }
}
