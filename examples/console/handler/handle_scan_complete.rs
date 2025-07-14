use ezsp::parameters::networking::handler::ScanComplete;
use log::{error, info};

/// Handle the scan complete event.
pub fn handle_scan_complete(scan_complete: &ScanComplete) {
    info!("Scan completed.");

    if let Some(channel) = scan_complete.channel() {
        error!("Scan failed on channel: {channel:#04X}");
    } else {
        info!("Scan succeeded.");
    }
}
