use crate::ember::{Eui64, Status};
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0090;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    broadcast: bool,
    dest_eui64: Eui64,
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(broadcast: bool, dest_eui64: Eui64, message: ByteSizedVec<u8>) -> Self {
        Self {
            broadcast,
            dest_eui64,
            message,
        }
    }

    #[must_use]
    pub const fn broadcast(&self) -> bool {
        self.broadcast
    }

    #[must_use]
    pub const fn dest_eui64(&self) -> Eui64 {
        self.dest_eui64
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
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
