use ezsp::parameters::networking::handler::NetworkFound;
use log::info;

use crate::utils::print_network;

/// Handle the network found event.
pub fn handle_network_found(network_found: &NetworkFound) {
    info!(
        "Network found: last hop RSSI: {}, last hop LQI: {}",
        network_found.last_hop_lqi(),
        network_found.last_hop_lqi()
    );
    print_network(network_found.network_found());
}
