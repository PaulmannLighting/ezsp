use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0061;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    node_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    eui64: Eui64,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl TryFrom<Response> for Eui64 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(response.eui64)
                } else {
                    Err(status.into())
                }
            })
    }
}
