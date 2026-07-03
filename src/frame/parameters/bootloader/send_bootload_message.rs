//! Parameters for the [`Bootloader::send_bootload_message()`](crate::Bootloader::send_bootload_message) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, Status};
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x0090, { broadcast: bool, dest_eui64: Eui64, message: ByteSizedVec<u8> }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(broadcast: bool, dest_eui64: Eui64, message: ByteSizedVec<u8>) -> Self {
        Self {
            broadcast,
            dest_eui64,
            message,
        }
    }
}

/// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
