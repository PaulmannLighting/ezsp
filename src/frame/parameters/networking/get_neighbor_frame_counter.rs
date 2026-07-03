//! Parameters for the [`Networking::get_neighbor_frame_counter`](crate::Networking::get_neighbor_frame_counter) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, Status};

crate::frame::parameters::frame!(0x003E, { eui64: Eui64 }, { status: u8, return_frame_counter: u32 });

impl Command {
    #[must_use]
    pub const fn new(eui64: Eui64) -> Self {
        Self { eui64 }
    }
}

/// Convert a response into a [`u32`] representing the return frame counter
/// or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for u32 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.return_frame_counter),
            other => Err(other.into()),
        }
    }
}
