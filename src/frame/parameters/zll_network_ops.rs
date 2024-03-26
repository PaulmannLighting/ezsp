use crate::ember::zll::{Network, NetworkOperation};
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00B2;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    #[must_use]
    pub const fn network_info(&self) -> &Network {
        &self.network_info
    }

    pub fn op(&self) -> Result<NetworkOperation, u8> {
        NetworkOperation::try_from(self.op)
    }

    #[must_use]
    pub const fn radio_tx_power(&self) -> i8 {
        self.radio_tx_power
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
