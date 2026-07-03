//! Parameters for the [`TrustCenter::broadcast_next_network_key`](crate::TrustCenter::broadcast_next_network_key) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::key::Data;

crate::frame::parameters::frame!(0x0073, { key: Data }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(key: Data) -> Self {
        Self { key }
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
