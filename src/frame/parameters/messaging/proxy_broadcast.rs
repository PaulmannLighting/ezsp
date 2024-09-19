use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::aps::Frame;
use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0037;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
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
            content,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    aps_sequence: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = u8;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.aps_sequence)
    }
}
