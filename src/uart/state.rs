use core::fmt::Debug;

use log::trace;

use crate::constants::MIN_NON_LEGACY_VERSION;
use crate::uart::connection::Connection;

/// Shared state of the `EZSP` UART.
#[derive(Debug, Default)]
pub struct State {
    negotiated_version: Option<u8>,
    connection: Connection,
}

impl State {
    /// Returns the negotiated version.
    #[must_use]
    pub const fn negotiated_version(&self) -> Option<u8> {
        self.negotiated_version
    }

    /// Set the negotiated version.
    pub const fn set_negotiated_version(&mut self, version: u8) {
        self.negotiated_version.replace(version);
    }

    /// Returns the connection state of the UART.
    #[must_use]
    pub const fn connection(&self) -> Connection {
        self.connection
    }

    /// Set the connection state of the UART.
    pub fn set_connection(&mut self, connection: Connection) {
        trace!("Setting connection state to: {connection:?}");
        self.connection = connection;

        if connection != Connection::Connected {
            trace!("Resetting negotiated version.");
            self.negotiated_version.take();
        }
    }

    /// Returns `true` if the negotiated version is a legacy version.
    #[must_use]
    pub fn is_legacy(&self) -> bool {
        self.negotiated_version
            .is_none_or(|version| version < MIN_NON_LEGACY_VERSION)
    }
}
