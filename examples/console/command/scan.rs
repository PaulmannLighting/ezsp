//! Perform an active scan on the network.

use log::{error, info};

use ezsp::ezsp::network::scan::Type;
use ezsp::uart::Uart;
use ezsp::Networking;

/// Perform an active scan on the network.
pub async fn scan(uart: &mut Uart, channel_mask: u32, scan_duration: u8) {
    match uart
        .start_scan(Type::ActiveScan, channel_mask, scan_duration)
        .await
    {
        Ok(()) => {
            info!("Started a scan");
        }
        Err(error) => {
            error!("Error starting scan: {error}");
        }
    }
}
