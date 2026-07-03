//! Parameters for the [`Networking::start_scan`](crate::Networking::start_scan) command.

use num_traits::FromPrimitive;
use silizium::Status;

use crate::Error;
use crate::ezsp::network::scan::Type;
use crate::types::VariableLengthU32;

crate::frame::parameters::frame!(0x001A, { scan_type: u8, channel_mask: u32, duration: u8 }, { status: VariableLengthU32 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub fn new(scan_type: Type, channel_mask: u32, duration: u8) -> Self {
        Self {
            scan_type: scan_type.into(),
            channel_mask,
            duration,
        }
    }
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let status = response.status.into();
        match Status::from_u32(status).ok_or(status) {
            Ok(Status::Ok) => Ok(()),
            other => Err(other.into()),
        }
    }
}
