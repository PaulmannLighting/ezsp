use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberStatus};

pub const ID: u16 = 0x0096;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(message_length: u8, message_contents: ByteSizedVec<u8>) -> Self {
        Self { message_length, message_contents }
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

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
