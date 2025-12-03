//! Parameters for the [`Security::set_initial_security_state`](crate::Security::set_initial_security_state) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::security::initial::State;
use crate::frame::Parameter;

const ID: u16 = 0x0068;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    state: State,
}

impl Command {
    #[must_use]
    pub const fn new(state: State) -> Self {
        Self { state }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    success: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.success).ok_or(response.success) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
