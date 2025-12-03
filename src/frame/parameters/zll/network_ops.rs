//! Parameters for the [`Zll::network_ops`](crate::Zll::network_ops) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::zll::Network;
use crate::ezsp::zll::NetworkOperation;
use crate::frame::Parameter;

const ID: u16 = 0x00B2;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    network_info: Network,
    op: u8,
    radio_tx_power: i8,
}

impl Command {
    #[must_use]
    pub fn new(network_info: Network, op: NetworkOperation, radio_tx_power: i8) -> Self {
        Self {
            network_info,
            op: op.into(),
            radio_tx_power,
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

/// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
