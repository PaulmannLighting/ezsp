//! Parameters for the [`Messaging::poll_for_data`](crate::Messaging::poll_for_data) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::event::Units;
use crate::frame::Parameter;

const ID: u16 = 0x0042;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    interval: u16,
    units: u8,
    failure_limit: u8,
}

impl Command {
    #[must_use]
    pub fn new(interval: u16, units: Units, failure_limit: u8) -> Self {
        Self {
            interval,
            units: units.into(),
            failure_limit,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
