//! Parameters for the [`GreenPower::sink_commission`](crate::GreenPower::sink_commission) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x010A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    options: u8,
    gpm_addr_for_security: u16,
    gpm_addr_for_pairing: u16,
    sink_endpoint: u8,
}

impl Command {
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
