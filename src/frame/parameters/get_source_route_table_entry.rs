use crate::ember::{NodeId, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00C1;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    destination: NodeId,
    closer_index: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, destination: NodeId, closer_index: u8) -> Self {
        Self {
            status: status.into(),
            destination,
            closer_index,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
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
