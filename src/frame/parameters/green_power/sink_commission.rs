//! Parameters for the [`GreenPower::sink_commission`](crate::GreenPower::sink_commission) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(0x010A, { options: u8, gpm_addr_for_security: u16, gpm_addr_for_pairing: u16, sink_endpoint: u8 }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(
        options: u8,
        gpm_addr_for_security: u16,
        gpm_addr_for_pairing: u16,
        sink_endpoint: u8,
    ) -> Self {
        Self {
            options,
            gpm_addr_for_security,
            gpm_addr_for_pairing,
            sink_endpoint,
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
