//! Parameters for the  [`Networking::set_duty_cycle_limits_in_stack`](crate::Networking::set_duty_cycle_limits_in_stack) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::duty_cycle::Limits;

crate::frame::parameters::frame!(0x0040, { limits: Limits }, { status: u8 });

impl From<Limits> for Command {
    fn from(limits: Limits) -> Self {
        Self { limits }
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
