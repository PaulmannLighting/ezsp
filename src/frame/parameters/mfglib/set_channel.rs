//! Parameters for the [`Mfglib::set_channel`](crate::Mfglib::set_channel) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(0x008A, { channel: u8 }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(channel: u8) -> Self {
        Self { channel }
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
