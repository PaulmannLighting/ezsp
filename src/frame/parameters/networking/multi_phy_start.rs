//! Parameters for the [`Networking::multi_phy_start`](crate::Networking::multi_phy_start) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::multi_phy::nwk::Config;
use crate::frame::Parameter;

const ID: u16 = 0x00F8;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    phy_index: u8,
    page: u8,
    channel: u8,
    power: i8,
    bitmask: u8,
}

impl Command {
    #[must_use]
    pub fn new(phy_index: u8, page: u8, channel: u8, power: i8, bitmask: Config) -> Self {
        Self {
            phy_index,
            page,
            channel,
            power,
            bitmask: bitmask.into(),
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert a response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
