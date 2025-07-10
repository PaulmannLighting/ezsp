//! Perform a token factory reset.

use ezsp::TokenInterface;
use ezsp::uart::Uart;
use log::{error, info};
use serialport::SerialPort;

/// Perform a token factory reset.
pub async fn token_factory_reset<T>(
    uart: &mut Uart<T>,
    exclude_outgoing_frame_counter: bool,
    exclude_boot_counter: bool,
) where
    T: SerialPort + 'static,
{
    match uart
        .token_factory_reset(exclude_outgoing_frame_counter, exclude_boot_counter)
        .await
    {
        Ok(()) => {
            info!("Token factory reset successful");
        }
        Err(error) => {
            error!("{error}");
        }
    }
}
