//! Parameters for the [`Messaging::send_raw_message_extended`](crate::Messaging::send_raw_message_extended) command.

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0051;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    message: Prefixed<u8, ByteSizedVec<u8>>,
    priority: u8,
    use_cca: bool,
}

impl Command {
    #[must_use]
    pub fn new(message: ByteSizedVec<u8>, priority: u8, use_cca: bool) -> Self {
        Self {
            message: message.into(),
            priority,
            use_cca,
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
