use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberNodeId,EmberApsFrame,EmberStatus};

pub const ID: u16 = 0x0036;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    destination: EmberNodeId,
    aps_frame: EmberApsFrame,
    radius: u8,
    message_tag: u8,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(destination: EmberNodeId, aps_frame: EmberApsFrame, radius: u8, message_tag: u8, message_length: u8, message_contents: ByteSizedVec<u8>) -> Self {
        Self { destination, aps_frame, radius, message_tag, message_length, message_contents }
    }

    #[must_use]
    pub const fn destination(&self) -> EmberNodeId {
        self.destination
    }


    #[must_use]
    pub const fn aps_frame(&self) -> EmberApsFrame {
        self.aps_frame
    }


    #[must_use]
    pub const fn radius(&self) -> u8 {
        self.radius
    }


    #[must_use]
    pub const fn message_tag(&self) -> u8 {
        self.message_tag
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
    sequence: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, sequence: u8) -> Self {
        Self { status, sequence }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }
}
