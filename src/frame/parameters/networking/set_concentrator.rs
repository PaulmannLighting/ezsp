//! Parameters for the [`Networking::set_concentrator`](crate::Networking::set_concentrator) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::concentrator::Parameters;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0010;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    on: bool,
    parameters: Parameters,
}

impl From<Option<Parameters>> for Command {
    fn from(parameters: Option<Parameters>) -> Self {
        Self {
            on: parameters.is_some(),
            parameters: parameters.unwrap_or_default(),
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
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
