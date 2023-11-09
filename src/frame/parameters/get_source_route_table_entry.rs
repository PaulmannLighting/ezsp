use crate::types::{EmberNodeId, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00C1;

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
    status: EmberStatus,
    destination: EmberNodeId,
    closer_index: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, destination: EmberNodeId, closer_index: u8) -> Self {
        Self {
            status,
            destination,
            closer_index,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn destination(&self) -> EmberNodeId {
        self.destination
    }

    #[must_use]
    pub const fn closer_index(&self) -> u8 {
        self.closer_index
    }
}
