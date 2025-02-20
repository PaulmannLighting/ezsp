//! Parameters for the [`Messaging::unicast_current_network_key`](crate::Messaging::unicast_current_network_key) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0050;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    target_short: NodeId,
    target_long: Eui64,
    parent_short_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(target_short: NodeId, target_long: Eui64, parent_short_id: NodeId) -> Self {
        Self {
            target_short,
            target_long,
            parent_short_id,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
