#![cfg(feature = "ashv2")]
//! ASHv2 extensions for the builder.

use std::io;

use ashv2::{Actor, SerialPort, TryCloneNative};
use tokio::sync::mpsc::channel;

use crate::uart::{Buffers, Uart};
use crate::zigbee::network_manager::builder::Builder;
use crate::{Error, MIN_NON_LEGACY_VERSION};

impl Builder<Uart> {
    /// Create a new builder using an `ASHv2` UART on the given serial port.
    pub fn ashv2<T>(serial_port: T, buffers: Buffers) -> Result<Self, Error>
    where
        T: SerialPort + TryCloneNative + Sync + 'static,
    {
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
