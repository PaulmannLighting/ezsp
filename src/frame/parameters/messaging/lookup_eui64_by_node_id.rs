//! Parameters for the [`Messaging::lookup_eui64_by_node_id`](crate::Messaging::lookup_eui64_by_node_id) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, NodeId, Status};

crate::frame::parameters::frame!(0x0061, { node_id: NodeId }, { status: u8, eui64: Eui64 });

impl Command {
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }
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
