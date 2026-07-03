//! Parameters for the [`Networking::permit_joining`](crate::Networking::permit_joining) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::network::Duration;

crate::frame::parameters::frame!(0x0022, { duration: u8 }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self {
            duration: duration.into(),
        }
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
