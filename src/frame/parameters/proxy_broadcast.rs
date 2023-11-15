use crate::ember::aps::Frame;
use crate::ember::{NodeId, Status};
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0037;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    source: NodeId,
    destination: NodeId,
    nwk_sequence: u8,
    aps_frame: Frame,
    radius: u8,
    message_tag: u8,
    content: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        source: NodeId,
        destination: NodeId,
        nwk_sequence: u8,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        content: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            source,
            destination,
            nwk_sequence,
            aps_frame,
            radius,
            message_tag,
            content: content,
        }
    }

    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    #[must_use]
    pub const fn destination(&self) -> NodeId {
        self.destination
    }

    #[must_use]
    pub const fn nwk_sequence(&self) -> u8 {
        self.nwk_sequence
    }

    #[must_use]
    pub const fn aps_frame(&self) -> &Frame {
        &self.aps_frame
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
    pub const fn content(&self) -> &ByteSizedVec<u8> {
        &self.content
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    aps_sequence: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, aps_sequence: u8) -> Self {
        Self {
            status: status.into(),
            aps_sequence,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn aps_sequence(&self) -> u8 {
        self.aps_sequence
    }
}
