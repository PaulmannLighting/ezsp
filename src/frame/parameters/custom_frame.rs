use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0047;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    payload_length: u8,
    payload: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(payload_length: u8, payload: ByteSizedVec<u8>) -> Self {
        Self {
            payload_length,
            payload,
        }
    }

    #[must_use]
    pub const fn payload_length(&self) -> u8 {
        self.payload_length
    }

    #[must_use]
    pub const fn payload(&self) -> &ByteSizedVec<u8> {
        &self.payload
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    reply_length: u8,
    reply: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, reply_length: u8, reply: ByteSizedVec<u8>) -> Self {
        Self {
            status: status.into(),
            reply_length,
            reply,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn reply_length(&self) -> u8 {
        self.reply_length
    }

    #[must_use]
    pub const fn reply(&self) -> &ByteSizedVec<u8> {
        &self.reply
    }
}
