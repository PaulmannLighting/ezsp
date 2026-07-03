//! Parameters for the [`Networking::set_child_data`](crate::Networking::set_child_data) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Status, child};

crate::frame::parameters::frame!(0x00AC, { index: u8, child_data: child::Data }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(index: u8, child_data: child::Data) -> Self {
        Self { index, child_data }
    }
}

/// Convert a response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
