use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::aps::Frame;
use crate::ember::message::Outgoing;
use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::{Error, ValueError};

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

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    sequence: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for u8 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(response.sequence)
                } else {
                    Err(status.into())
                }
            })
    }
}
