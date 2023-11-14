use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0089;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    packet_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(packet_contents: ByteSizedVec<u8>) -> Self {
        Self { packet_contents }
    }

    #[must_use]
    pub const fn packet_contents(&self) -> &ByteSizedVec<u8> {
        &self.packet_contents
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
