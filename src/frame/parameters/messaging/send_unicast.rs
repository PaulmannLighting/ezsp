use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::aps::Frame;
use crate::ember::message::Outgoing;
use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Resolve;
use crate::{frame, Error};

const ID: u16 = 0x0034;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    sequence: u8,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = u8;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.sequence)
    }
}
