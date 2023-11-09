use crate::types::{ByteSizedVec, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0093;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberStatus,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(
        status: EmberStatus,
        message_length: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            status,
            message_length,
            message_contents,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }

    #[must_use]
    pub const fn message_contents(&self) -> ByteSizedVec<u8> {
        self.message_contents
    }
}
