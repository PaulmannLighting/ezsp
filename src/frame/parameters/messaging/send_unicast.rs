use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::aps::Frame;
use crate::ember::message::Outgoing;
use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0034;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    typ: u8,
    index_or_destination: NodeId,
    aps_frame: Frame,
    tag: u8,
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub fn new(
        typ: Outgoing,
        index_or_destination: NodeId,
        aps_frame: Frame,
        tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            typ: typ.into(),
            index_or_destination,
            aps_frame,
            tag,
            message,
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
