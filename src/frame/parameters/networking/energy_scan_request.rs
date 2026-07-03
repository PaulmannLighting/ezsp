//! Parameters for the [`Networking::energy_scan_request`](crate::Networking::energy_scan_request) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{NodeId, Status};

crate::frame::parameters::frame!(0x009C, { target: NodeId, scan_channels: u32, scan_duration: u8, scan_count: u16 }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(
        target: NodeId,
        scan_channels: u32,
        scan_duration: u8,
        scan_count: u16,
    ) -> Self {
        Self {
            target,
            scan_channels,
            scan_duration,
            scan_count,
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
