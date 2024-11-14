//! Perform a token factory reset.

use log::{error, info};

use ezsp::uart::Uart;
use ezsp::TokenInterface;

/// Perform a token factory reset.
pub async fn token_factory_reset(
    uart: &mut Uart,
    exclude_outgoing_frame_counter: bool,
    exclude_boot_counter: bool,
) {
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
