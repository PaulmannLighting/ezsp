//! Parameters for the [`Networking::get_first_beacon`](crate::Networking::get_first_beacon) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::beacon::Iterator;
use crate::frame::Parameter;

const ID: u16 = 0x003D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    beacon_iterator: Iterator,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into [`Iterator`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Iterator {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.beacon_iterator),
            other => Err(other.into()),
        }
    }
}
