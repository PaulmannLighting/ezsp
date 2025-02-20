use le_stream::derive::FromLeStream;

use crate::ember::zll::Network;
use crate::frame::Parameter;

const ID: u16 = 0x00BB;

/// This call is fired when the device is a target of a touch link.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    network_info: Network,
}

impl Handler {
    /// Information about the network.
    #[must_use]
    pub const fn network_info(&self) -> &Network {
        &self.network_info
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
