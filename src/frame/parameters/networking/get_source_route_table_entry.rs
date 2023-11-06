use crate::ember::types::NodeId;
use crate::ember::Status;
use crate::Error;
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

impl TryFrom<&[u8]> for Response {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() == 4 {
            Ok(Self::new(
                Status::try_from(bytes[0])?,
                NodeId::from_le_bytes([bytes[1], bytes[2]]),
                bytes[3],
            ))
        } else {
            Err(Error::InvalidSize {
                expected: 4,
                found: bytes.len(),
            })
        }
    }
}
