//! Perform an active scan on the network.

use ashv2::SerialPort;
use ezsp::Networking;
use ezsp::ezsp::network::scan::Type;
use ezsp::uart::Uart;
use log::{error, info};

/// Perform an active scan on the network.
pub async fn scan<T>(uart: &mut Uart<T>, channel_mask: u32, scan_duration: u8)
where
    T: SerialPort + 'static,
{
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
