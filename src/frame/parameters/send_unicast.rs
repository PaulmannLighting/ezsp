use crate::ember::aps::Frame;
use crate::ember::message::Outgoing;
use crate::ember::{NodeId, Status};
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0034;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    typ: Outgoing,
    index_or_destination: NodeId,
    aps_frame: Frame,
    tag: u8,
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        typ: Outgoing,
        index_or_destination: NodeId,
        aps_frame: Frame,
        tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            typ,
            index_or_destination,
            aps_frame,
            tag,
            message,
        }
    }

    #[must_use]
    pub const fn typ(&self) -> Outgoing {
        self.typ
    }

    #[must_use]
    pub const fn index_or_destination(&self) -> NodeId {
        self.index_or_destination
    }

    #[must_use]
    pub const fn aps_frame(&self) -> &Frame {
        &self.aps_frame
    }

    #[must_use]
    pub const fn tag(&self) -> u8 {
        self.tag
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
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
