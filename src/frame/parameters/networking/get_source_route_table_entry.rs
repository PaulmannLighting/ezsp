use crate::ember::types::NodeId;
use crate::ember::Status;
use crate::read_write::Readable;
use crate::Error;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{empty, Empty};

pub const ID: u16 = 0x00C1;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        empty()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    destination: NodeId,
    closer_index: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, destination: NodeId, closer_index: u8) -> Self {
        Self {
            status,
            destination,
            closer_index,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn destination(&self) -> NodeId {
        self.destination
    }

    #[must_use]
    pub const fn closer_index(&self) -> u8 {
        self.closer_index
    }
}

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        let destination: NodeId = src.read_num_le()?;
        let closer_index: u8 = src.read_num_le()?;
        Ok(Self::new(status.try_into()?, destination, closer_index))
    }
}
