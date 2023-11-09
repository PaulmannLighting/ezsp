use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool,EmberEUI64,EmberStatus};

pub const ID: u16 = 0x0090;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    broadcast: bool,
    dest_eui64: EmberEUI64,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(broadcast: bool, dest_eui64: EmberEUI64, message_length: u8, message_contents: ByteSizedVec<u8>) -> Self {
        Self { broadcast, dest_eui64, message_length, message_contents }
    }

    #[must_use]
    pub const fn broadcast(&self) -> bool {
        self.broadcast
    }


    #[must_use]
    pub const fn dest_eui64(&self) -> EmberEUI64 {
        self.dest_eui64
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
