//! Parameters for the [`Security::get_current_security_state`](crate::Security::get_current_security_state) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::security::current::State;
use crate::frame::Parameter;

const ID: u16 = 0x0069;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    state: State,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into [`State`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for State {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.state),
            other => Err(other.into()),
        }
    }
}
