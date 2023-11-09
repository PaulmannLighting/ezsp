use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberNodeId,EmberApsFrame,EmberStatus};

pub const ID: u16 = 0x0039;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    sender: EmberNodeId,
    aps_frame: EmberApsFrame,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(sender: EmberNodeId, aps_frame: EmberApsFrame, message_length: u8, message_contents: ByteSizedVec<u8>) -> Self {
        Self { sender, aps_frame, message_length, message_contents }
    }

    #[must_use]
    pub const fn sender(&self) -> EmberNodeId {
        self.sender
    }


    #[must_use]
    pub const fn aps_frame(&self) -> EmberApsFrame {
        self.aps_frame
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
