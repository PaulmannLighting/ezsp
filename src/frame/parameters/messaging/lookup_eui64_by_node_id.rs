//! Parameters for the [`Messaging::lookup_eui64_by_node_id`](crate::Messaging::lookup_eui64_by_node_id) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;

const ID: u16 = 0x0061;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    node_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    eui64: Eui64,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into the [`Eui64`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Eui64 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.eui64),
            other => Err(other.into()),
        }
    }
}
