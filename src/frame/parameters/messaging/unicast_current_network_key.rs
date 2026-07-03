//! Parameters for the [`Messaging::unicast_current_network_key`](crate::Messaging::unicast_current_network_key) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, NodeId, Status};

crate::frame::parameters::frame!(0x0050, { target_short: NodeId, target_long: Eui64, parent_short_id: NodeId }, { status: u8 });

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
