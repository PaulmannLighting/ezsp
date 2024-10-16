use crate::ember::zll::Network;
use crate::ember::Status;
use crate::ezsp::zll::NetworkOperation;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

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
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> crate::Result<Self::Output> {
        Status::try_from(self.status).resolve()
    }
}
