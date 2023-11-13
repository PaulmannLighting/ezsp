use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0047;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    payload: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(payload: ByteSizedVec<u8>) -> Self {
        Self { payload }
    }

    #[must_use]
    pub const fn payload(&self) -> &ByteSizedVec<u8> {
        &self.payload
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    reply: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, reply: ByteSizedVec<u8>) -> Self {
        Self {
            status: status.into(),
            reply,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn reply(&self) -> &ByteSizedVec<u8> {
        &self.reply
    }
}
