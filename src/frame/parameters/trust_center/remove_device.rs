//! Parameters for the [`TrustCenter::remove_device`](crate::TrustCenter::remove_device) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, NodeId, Status};

crate::frame::parameters::frame!(0x00A8, { dest_short: NodeId, dest_long: Eui64, target_long: Eui64 }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(dest_short: NodeId, dest_long: Eui64, target_long: Eui64) -> Self {
        Self {
            dest_short,
            dest_long,
            target_long,
        }
    }
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
