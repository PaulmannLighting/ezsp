//! Callback handler implementation.

use ezsp::{Callback, parameters};
use log::{debug, warn};

use handle_network_found::handle_network_found;
use handle_scan_complete::handle_scan_complete;

mod handle_network_found;
mod handle_scan_complete;

/// A stub callback handler.
pub struct Handler;

impl ezsp::Handler for Handler {
    fn handle(&mut self, callback: Callback) {
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
}
