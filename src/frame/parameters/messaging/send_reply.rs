//! Parameters for the [`Messaging::send_reply`](crate::Messaging::send_reply) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::aps::Frame;
use crate::ember::{NodeId, Status};
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x0039, { sender: NodeId, aps_frame: Frame, message: ByteSizedVec<u8> }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(sender: NodeId, aps_frame: Frame, message: ByteSizedVec<u8>) -> Self {
        Self {
            sender,
            aps_frame,
            message,
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
