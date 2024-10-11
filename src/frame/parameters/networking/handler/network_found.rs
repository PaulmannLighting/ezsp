use le_stream::derive::FromLeStream;

use crate::ember::zigbee::Network;
use crate::frame::Parameter;

const ID: u16 = 0x001B;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    network_found: Network,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Handler {
    #[must_use]
    pub const fn network_found(&self) -> &Network {
        &self.network_found
    }

    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
}

impl Parameter<u16> for Handler {
    const ID: u16 = ID;
}
