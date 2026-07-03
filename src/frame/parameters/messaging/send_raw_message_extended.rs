//! Parameters for the [`Messaging::send_raw_message_extended`](crate::Messaging::send_raw_message_extended) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x0051, { message: ByteSizedVec<u8>, priority: u8, use_cca: bool }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(message: ByteSizedVec<u8>, priority: u8, use_cca: bool) -> Self {
        Self {
            message,
            priority,
            use_cca,
        }
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
