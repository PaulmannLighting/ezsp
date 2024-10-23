use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::zll::Network;
use crate::ember::Status;
use crate::ezsp::zll::NetworkOperation;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x00B2;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
