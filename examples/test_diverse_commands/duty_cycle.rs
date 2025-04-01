//! Test duty-cycle-related commands.

use log::{error, info};

use le_stream::ToLeStream;

use ezsp::Networking;
use ezsp::uart::Uart;

/// Test duty-cycle-related commands.
pub async fn get_duty_cycle_info(ezsp: &mut Uart) {
    match ezsp.get_duty_cycle_limits().await {
        Ok(limits) => {
            info!("Duty cycle limits: {limits:#04X?}");
            info!("Duty cycle limits size: {}", limits.to_le_stream().count());
        }
        Err(error) => {
            error!("Error getting duty cycle limits: {error}");
        }
    }

    match ezsp.get_duty_cycle_state().await {
        Ok(state) => {
            info!("Duty cycle state: {state:#04X?}");
        }
        Err(error) => {
            error!("Error getting duty cycle state: {error}");
        }
    }

    match ezsp.get_current_duty_cycle(10).await {
        Ok(duty_cycles) => {
            info!("Current duty cycles: {duty_cycles:?}");
        }
        Err(error) => {
            error!("Error getting current duty cycles: {error}");
        }
    }
}
