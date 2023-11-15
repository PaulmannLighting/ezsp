use crate::ember::aps::Frame;
use crate::ember::{NodeId, Status};
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0036;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    destination: NodeId,
    aps_frame: Frame,
    radius: u8,
    message_tag: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        destination: NodeId,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            destination,
            aps_frame,
            radius,
            message_tag,
            message_contents,
        }
    }

    #[must_use]
    pub const fn destination(&self) -> NodeId {
        self.destination
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
    pub const fn message_contents(&self) -> &ByteSizedVec<u8> {
        &self.message_contents
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    sequence: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, sequence: u8) -> Self {
        Self {
            status: status.into(),
            sequence,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn sequence(&self) -> u8 {
        self.sequence
    }
}
