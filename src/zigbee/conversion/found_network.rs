use zigbee_nwk::FoundNetwork;

use crate::parameters::networking::handler::NetworkFound;

impl From<NetworkFound> for FoundNetwork {
    fn from(network_found: NetworkFound) -> Self {
        let network = network_found.network_found();
        Self::new(
            zigbee_nwk::Network::new(
                network.channel(),
                network.pan_id(),
                network.extended_pan_id(),
                network.allowing_join(),
                network.stack_profile(),
                network.nwk_update_id(),
            ),
            network_found.last_hop_lqi(),
            network_found.last_hop_rssi(),
        )
    }
}
