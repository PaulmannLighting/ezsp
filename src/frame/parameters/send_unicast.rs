use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberOutgoingMessageType,EmberNodeId,EmberApsFrame,EmberStatus};

pub const ID: u16 = 0x0034;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    typ: EmberOutgoingMessageType,
    index_or_destination: EmberNodeId,
    aps_frame: EmberApsFrame,
    message_tag: u8,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(typ: EmberOutgoingMessageType, index_or_destination: EmberNodeId, aps_frame: EmberApsFrame, message_tag: u8, message_length: u8, message_contents: ByteSizedVec<u8>) -> Self {
        Self { typ, index_or_destination, aps_frame, message_tag, message_length, message_contents }
    }

    #[must_use]
    pub const fn typ(&self) -> EmberOutgoingMessageType {
        self.typ
    }


    #[must_use]
    pub const fn index_or_destination(&self) -> EmberNodeId {
        self.index_or_destination
    }


    #[must_use]
    pub const fn aps_frame(&self) -> EmberApsFrame {
        self.aps_frame
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
