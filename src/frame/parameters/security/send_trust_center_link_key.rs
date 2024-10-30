//! Parameters for the [`Security::send_trust_center_link_key`](crate::Security::send_trust_center_link_key) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0067;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    destination_node_id: NodeId,
    destination_eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(destination_node_id: NodeId, destination_eui64: Eui64) -> Self {
        Self {
            destination_node_id,
            destination_eui64,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
