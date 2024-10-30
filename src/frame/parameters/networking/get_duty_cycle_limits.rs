//! Parameters for the [`Networking::get_duty_cycle_limits`](crate::Networking::get_duty_cycle_limits) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::duty_cycle::Limits;
use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x004B;

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
    returned_limits: Limits,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into [`Limits`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Limits {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.returned_limits),
            other => Err(other.into()),
        }
    }
}
