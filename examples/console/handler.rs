//! Callback handler implementation.

use ezsp::{Callback, parameters};
use handle_network_found::handle_network_found;
use handle_scan_complete::handle_scan_complete;
use log::{debug, warn};

mod handle_network_found;
mod handle_scan_complete;

pub fn handle_callback(callback: Callback) {
    debug!("Handling callback.");

    match callback {
        Callback::Networking(parameters::networking::handler::Handler::NetworkFound(
            network_found,
        )) => {
            handle_network_found(&network_found);
        }
        Callback::Networking(parameters::networking::handler::Handler::ScanComplete(
            scan_complete,
        )) => {
            handle_scan_complete(&scan_complete);
        }
        other => {
            warn!("Received unexpected handler: {other:?}");
        }
    }
}
