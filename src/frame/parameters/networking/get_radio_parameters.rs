//! Parameters for the [`Networking::get_radio_parameters`](crate::Networking::get_radio_parameters) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::multi_phy::radio::Parameters;

crate::frame::parameters::frame!(0x00FD, { phy_index: u8 }, { status: u8, parameters: Parameters });

impl Command {
    #[must_use]
    pub const fn new(phy_index: u8) -> Self {
        Self { phy_index }
    }
}

/// Converts the response into [`Parameters`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Parameters {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.parameters),
            other => Err(other.into()),
        }
    }
}
