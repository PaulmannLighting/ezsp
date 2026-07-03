//! Parameters for the [`Networking::network_init`](crate::Networking::network_init) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ezsp::network::InitBitmask;

crate::frame::parameters::frame!(0x0017, { bitmask: InitBitmask }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(bitmask: InitBitmask) -> Self {
        Self { bitmask }
    }
}

/// Convert a response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
