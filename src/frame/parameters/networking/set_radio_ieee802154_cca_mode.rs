//! Parameters for the [`Networking::set_radio_ieee802154_cca_mode`](crate::Networking::set_radio_ieee802154_cca_mode) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;

crate::frame::parameters::frame!(0x0095, { cca_mode: u8 }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(cca_mode: u8) -> Self {
        Self { cca_mode }
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
