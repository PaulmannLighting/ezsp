use le_stream::derive::FromLeStream;

use crate::ember::zigbee::Network;
use crate::frame::Identified;

const ID: u16 = 0x001B;

/// Reports that a network was found as a result of a prior call
/// to [`Networking::start_scan`](crate::Networking::start_scan).
///
/// Gives the network parameters useful for deciding which network to join.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    network_found: Network,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Handler {
    /// The parameters associated with the network found.
    #[must_use]
    pub const fn network_found(&self) -> &Network {
        &self.network_found
    }

    /// The link quality from the node that generated this beacon.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// The energy level (in units of dBm) observed during the reception.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
