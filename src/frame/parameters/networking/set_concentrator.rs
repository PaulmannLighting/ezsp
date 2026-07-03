//! Parameters for the [`Networking::set_concentrator`](crate::Networking::set_concentrator) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::concentrator::Parameters;

crate::frame::parameters::frame!(0x0010, { on: bool, parameters: Parameters }, { status: u8 });

impl From<Option<Parameters>> for Command {
    fn from(parameters: Option<Parameters>) -> Self {
        Self {
            on: parameters.is_some(),
            parameters: parameters.unwrap_or_default(),
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
