use ezsp::Callback;
use ezsp::ember::zigbee::Network;
use ezsp::parameters::networking::handler::Handler;
use log::{debug, error, info, warn};

pub fn handle_callback(callback: Callback) {
    debug!("Handling callback.");

    match callback {
        Callback::Networking(Handler::NetworkFound(network_found)) => {
            info!(
                "Network found: last hop RSSI: {}, last hop LQI: {}",
                network_found.last_hop_lqi(),
                network_found.last_hop_lqi()
            );
            print_network(network_found.network_found());
        }
        Callback::Networking(Handler::ScanComplete(scan_complete)) => {
            info!("Scan completed.");

            if let Some(channel) = scan_complete.channel() {
                error!("Scan failed on channel: {:#04X}", channel);
            } else {
                info!("Scan succeeded.");
            }
        }
        other => {
            warn!("Received unexpected handler: {other:?}");
        }
    }
}

fn print_network(network: &Network) {
    info!("  * channel: {:#04X}", network.channel());
    info!("  * PAN ID: {:#06X}", network.pan_id());
    info!("  * Extended PAN ID: {}", network.extended_pan_id());
    info!("  * Allowing join: {}", network.allowing_join());
    info!("  * Stack profile: {}", network.stack_profile());
    info!("  * Update ID: {}", network.nwk_update_id());
}
