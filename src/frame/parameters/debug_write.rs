use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0012;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    binary_message: bool,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        binary_message: bool,
        message_length: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            binary_message,
            message_length,
            message_contents,
        }
    }

    #[must_use]
    pub const fn binary_message(&self) -> bool {
        self.binary_message
    }

    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }

    #[must_use]
    pub const fn message_contents(&self) -> &ByteSizedVec<u8> {
        &self.message_contents
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
