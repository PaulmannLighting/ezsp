//! Parameters for the [`Utilities::custom_frame`](crate::Utilities::custom_frame) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x0047, { payload: ByteSizedVec<u8> }, { status: u8, reply: ByteSizedVec<u8> });

impl Command {
    #[must_use]
    pub const fn new(payload: ByteSizedVec<u8>) -> Self {
        Self { payload }
    }
}

/// Converts the response into the reply payload or an error, depending on the status.
impl TryFrom<Response> for ByteSizedVec<u8> {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.reply),
            other => Err(other.into()),
        }
    }
}
