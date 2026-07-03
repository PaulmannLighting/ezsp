//! Parameters for the [`Mfglib::set_power`](crate::Mfglib::set_power) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(0x008C, { tx_power_mode: u16, power: i8 }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(tx_power_mode: u16, power: i8) -> Self {
        Self {
            tx_power_mode,
            power,
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
