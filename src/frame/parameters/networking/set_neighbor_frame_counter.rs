//! Parameters for then [`Networking::set_neighbor_frame_counter`](crate::Networking::set_neighbor_frame_counter) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, Status};

crate::frame::parameters::frame!(0x00AD, { eui64: Eui64, frame_counter: u32 }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(eui64: Eui64, frame_counter: u32) -> Self {
        Self {
            eui64,
            frame_counter,
        }
    }
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
