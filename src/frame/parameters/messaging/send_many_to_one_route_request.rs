//! Parameters for the [`Messaging::send_many_to_one_route_request`](crate::Messaging::send_many_to_one_route_request) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::concentrator::Type;
use crate::frame::Parameter;

const ID: u16 = 0x0041;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    concentrator_type: u16,
    radius: u8,
}

impl Command {
    #[must_use]
    pub const fn new(concentrator_type: Type, radius: u8) -> Self {
        Self {
            concentrator_type: concentrator_type as u16,
            radius,
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
