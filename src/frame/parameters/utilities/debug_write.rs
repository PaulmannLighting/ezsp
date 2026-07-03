//! Parameters for the [`Utilities::debug_write`](crate::Utilities::debug_write) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x0012, { binary_message: bool, message: ByteSizedVec<u8> }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(binary_message: bool, message: ByteSizedVec<u8>) -> Self {
        Self {
            binary_message,
            message,
        }
    }
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
