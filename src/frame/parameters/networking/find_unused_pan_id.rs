use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
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

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let channel_mask = src.read_num_be()?;
        let duration = src.read_num_be()?;
        Ok(Self {
            channel_mask,
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

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_be()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
