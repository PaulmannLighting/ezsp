//! Parameters for the [`Messaging::write_node_data`](crate::Messaging::write_node_data) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(0x00FE, { erase: bool }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(erase: bool) -> Self {
        Self { erase }
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
