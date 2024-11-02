//! Zigbee network manager implementation.

mod state;

use crate::zigbee::state::State;
use crate::Transport;

/// Zigbee network manager using the UART.
#[derive(Debug)]
pub struct EzspNetworkManager<T>
where
    T: Transport,
{
    transport: T,
    state: Option<State>,
}

impl<T> EzspNetworkManager<T>
where
    T: Transport,
{
    /// Creates a new network manager.
    #[must_use]
    pub const fn new(transport: T) -> Self {
        Self {
            transport,
            state: None,
        }
    }

    pub fn init(&mut self) {
        todo!("Init transport, query bootloader config, configuration values and policies.")
    }
}

impl<T> From<T> for EzspNetworkManager<T>
where
    T: Transport,
{
    fn from(transport: T) -> Self {
        Self::new(transport)
    }
}
