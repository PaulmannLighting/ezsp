#![cfg(feature = "ashv2")]
//! ASHv2 extensions for the builder.

use std::io;

use ashv2::{Actor, SerialPort, TryCloneNative};
use tokio::sync::mpsc::channel;

use crate::uart::Uart;
use crate::zigbee::network_manager::builder::Builder;
use crate::{Error, MIN_NON_LEGACY_VERSION};

/// Buffer sizes to set up the `ASHv2` communication channels.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Buffers {
    pub ash_receiver: usize,
    pub ash_transmitter: usize,
    pub ezsp_callbacks: usize,
    pub ezsp_messages: usize,
}

impl Default for Buffers {
    fn default() -> Self {
        Self {
            ash_receiver: 128,
            ash_transmitter: 128,
            ezsp_callbacks: 128,
            ezsp_messages: 128,
        }
    }
}

impl Builder<Uart> {
    /// Create a new builder using an `ASHv2` UART on the given serial port.
    pub fn ashv2<T>(serial_port: T, buffers: Option<Buffers>) -> Result<Self, Error>
    where
        T: SerialPort + TryCloneNative + Sync + 'static,
    {
        let buffers = buffers.unwrap_or_default();
        let (ash_tx, ash_rx) = channel(buffers.ash_receiver);
        let (_ashv2_tasks, proxy) = Actor::new(serial_port, ash_tx, buffers.ash_transmitter)
            .map_err(io::Error::from)?
            .spawn();
        let (callbacks_tx, callbacks_rx) = channel(buffers.ezsp_callbacks);
        let uart = Uart::new(
            proxy,
            ash_rx,
            callbacks_tx,
            MIN_NON_LEGACY_VERSION,
            buffers.ezsp_messages,
        );
        Ok(Self::new(uart, callbacks_rx))
    }
}
