//! Parameters for the [`Mfglib::start_stream`](crate::Mfglib::start_stream) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(0x0083, { rx_callback: bool }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(rx_callback: bool) -> Self {
        Self { rx_callback }
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
