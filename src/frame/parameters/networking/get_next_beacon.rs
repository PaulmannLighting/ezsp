//! Parameters for the [`Networking::get_next_beacon`](crate::Networking::get_next_beacon) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::beacon::Data;
use crate::frame::Parameter;

const ID: u16 = 0x0004;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    beacon: Data,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert a response into a [`Data`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Data {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.beacon),
            other => Err(other.into()),
        }
    }
}
