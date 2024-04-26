use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::aps::Frame;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x003A;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    sequence: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = u8;

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.sequence)
    }
}
