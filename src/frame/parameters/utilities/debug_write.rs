//! Parameters for the [`Utilities::debug_write`](crate::Utilities::debug_write) command.

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0012;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    binary_message: bool,
    message: Prefixed<u8, ByteSizedVec<u8>>,
}

impl Command {
    #[must_use]
    pub fn new(binary_message: bool, message: ByteSizedVec<u8>) -> Self {
        Self {
            binary_message,
            message: message.into(),
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

/// Converts the response into `()` or an error, depending on the status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
