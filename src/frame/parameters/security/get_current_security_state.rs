//! Parameters for the [`Security::get_current_security_state`](crate::Security::get_current_security_state) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::security::current::State;
use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0069;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    state: State,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into [`State`] or an appropriate error depending on its status.
impl TryFrom<Response> for State {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.state),
            other => Err(other.into()),
        }
    }
}
