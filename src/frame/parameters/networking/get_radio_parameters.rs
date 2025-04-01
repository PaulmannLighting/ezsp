//! Parameters for the [`Networking::get_radio_parameters`](crate::Networking::get_radio_parameters) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::multi_phy::radio::Parameters;
use crate::frame::Parameter;

const ID: u16 = 0x00FD;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    phy_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(phy_index: u8) -> Self {
        Self { phy_index }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    parameters: Parameters,
}

impl Parameter for Response {
    const ID: u16 = ID;
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
