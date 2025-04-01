//! Parameters for the [`Networking::get_child_data`](crate::Networking::get_child_data) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::child::Data;
use crate::frame::Parameter;

const ID: u16 = 0x004A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    child_data: Data,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into [`Data`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Data {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.child_data),
            other => Err(other.into()),
        }
    }
}
