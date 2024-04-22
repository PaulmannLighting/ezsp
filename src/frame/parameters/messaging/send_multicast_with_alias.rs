use crate::ember::aps::Frame;
use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x003A;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    aps_frame: Frame,
    hops: u8,
    nonmember_radius: u8,
    alias: u16,
    nwk_sequence: u8,
    message_tag: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        alias: u16,
        nwk_sequence: u8,
        message_tag: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            aps_frame,
            hops,
            nonmember_radius,
            alias,
            nwk_sequence,
            message_tag,
            message_contents,
        }
    }

    #[must_use]
    pub const fn aps_frame(&self) -> &Frame {
        &self.aps_frame
    }

    #[must_use]
    pub const fn hops(&self) -> u8 {
        self.hops
    }

    #[must_use]
    pub const fn nonmember_radius(&self) -> u8 {
        self.nonmember_radius
    }

    #[must_use]
    pub const fn alias(&self) -> u16 {
        self.alias
    }

    #[must_use]
    pub const fn nwk_sequence(&self) -> u8 {
        self.nwk_sequence
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
