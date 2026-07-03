//! Parameters for the [`Messaging::send_raw_message`](crate::Messaging::send_raw_message) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x0096, { message_contents: ByteSizedVec<u8> }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(message_contents: ByteSizedVec<u8>) -> Self {
        Self { message_contents }
    }
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
