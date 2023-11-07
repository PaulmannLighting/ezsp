use crate::ember::network::Status;
use crate::ember::types::Eui64;
use crate::read_write::Readable;
use crate::Error;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x003E;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: Eui64) -> Self {
        Self { eui64 }
    }

    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        self.eui64.to_le_bytes().into_iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    return_frame_counter: u32,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, return_frame_counter: u32) -> Self {
        Self {
            status,
            return_frame_counter,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn return_frame_counter(&self) -> u32 {
        self.return_frame_counter
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, IntoIter<Self::Item, 4>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into()).chain(self.return_frame_counter.to_le_bytes())
    }
}

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        let return_frame_counter = src.read_num_le()?;
        Ok(Self {
            status: status.into(),
            return_frame_counter,
        })
    }
}
