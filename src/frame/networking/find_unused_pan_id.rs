use crate::ember::Status;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0xD3;

/// This function starts a series of scans which will return an available panId.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    channel_mask: u32,
    duration: u8,
}

impl Command {
    #[must_use]
    pub const fn new(channel_mask: u32, duration: u8) -> Self {
        Self {
            channel_mask,
            duration,
        }
    }

    #[must_use]
    pub const fn channel_mask(&self) -> u32 {
        self.channel_mask
    }

    #[must_use]
    pub const fn duration(&self) -> u8 {
        self.duration
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<IntoIter<Self::Item, 4>, IntoIter<Self::Item, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.channel_mask
            .to_be_bytes()
            .into_iter()
            .chain(self.duration.to_be_bytes())
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer = [0; 5];
        src.read_exact(&mut buffer)?;
        let [channel_mask @ .., duration] = buffer;
        Ok(Self {
            channel_mask: u32::from_be_bytes(channel_mask),
            duration,
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

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer = [0; 1];
        src.read_exact(&mut buffer)?;
        let [status] = buffer;
        Ok(Self {
            status: Status::try_from(status)?,
        })
    }
}
