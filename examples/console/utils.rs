use ezsp::ember::zigbee::Network;
use log::info;

/// Print network information.
pub fn print_network(network: &Network) {
    info!("  * channel: {:#04X}", network.channel());
    info!("  * PAN ID: {:#06X}", network.pan_id());
    info!("  * Extended PAN ID: {}", network.extended_pan_id());
    info!("  * Allowing join: {}", network.allowing_join());
    info!("  * Stack profile: {}", network.stack_profile());
    info!("  * Update ID: {}", network.nwk_update_id());
}
