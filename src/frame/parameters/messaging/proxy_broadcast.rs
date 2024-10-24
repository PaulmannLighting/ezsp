use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::aps::Frame;
use crate::ember::{NodeId, Status};
use crate::frame::Identified;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0037;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    aps_sequence: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for u8 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.aps_sequence),
            other => Err(other.into()),
        }
    }
}
