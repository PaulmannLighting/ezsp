use le_stream::FromLeStream;

use crate::ember::zigbee::Network;
use crate::frame::Parameter;

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

impl Parameter for Handler {
    const ID: u16 = ID;
}

#[cfg(feature = "zigbee")]
impl From<Handler> for zigbee_nwk::FoundNetwork {
    fn from(handler: Handler) -> Self {
        zigbee_nwk::FoundNetwork::new(
            zigbee_nwk::Network::new(
                handler.network_found.channel(),
                handler.network_found.pan_id(),
                handler.network_found.extended_pan_id(),
                handler.network_found.allowing_join(),
                handler.network_found.stack_profile(),
                handler.network_found.nwk_update_id(),
            ),
            handler.last_hop_lqi(),
            handler.last_hop_rssi(),
        )
    }
}
