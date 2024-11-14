use std::fmt::Debug;
use std::sync::RwLock;

use log::trace;

use crate::frame::Disambiguation;
use crate::transport::MIN_NON_LEGACY_VERSION;
use crate::uart::connection::Connection;

/// Shared state of the `EZSP` UART.
#[derive(Debug)]
pub struct State {
    negotiated_version: RwLock<Option<u8>>,
    connection: RwLock<Connection>,
    disambiguation: RwLock<Option<Option<Disambiguation>>>,
}

impl State {
    /// Return the negotiated version.
    ///
    /// Returns `None` if the version has not been negotiated yet.
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn negotiated_version(&self) -> Option<u8> {
        *self
            .negotiated_version
            .read()
            .expect("RwLock should never be poisoned. This is a bug.")
    }

    /// Set the negotiated version.
    pub fn set_negotiated_version(&self, version: u8) {
        self.negotiated_version
            .write()
            .expect("RwLock should never be poisoned. This is a bug.")
            .replace(version);
    }

    /// Returns the connection state of the UART.
    #[must_use]
    pub fn connection(&self) -> Connection {
        *self
            .connection
            .read()
            .expect("RwLock should never be poisoned. This is a bug.")
    }

    /// Set the connection state of the UART.
    pub fn set_connection(&self, connection: Connection) {
        trace!("Setting connection state to: {connection:?}");
        *self
            .connection
            .write()
            .expect("RwLock should never be poisoned. This is a bug.") = connection;

        if connection != Connection::Connected {
            trace!("Resetting negotiated version.");
            self.negotiated_version
                .write()
                .expect("RwLock should never be poisoned. This is a bug.")
                .take();
        }
    }

    /// Returns `true` if the negotiated version is a legacy version.
    #[must_use]
    pub fn is_legacy(&self) -> bool {
        self.negotiated_version()
            .map_or(true, |version| version < MIN_NON_LEGACY_VERSION)
    }

    /// Returns the disambiguation.
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn disambiguation(&self) -> Option<Option<Disambiguation>> {
        *self
            .disambiguation
            .read()
            .expect("RwLock should never be poisoned. This is a bug.")
    }

    /// Set the disambiguation.
    pub fn set_disambiguation(&self, disambiguation: Option<Disambiguation>) {
        self.disambiguation
            .write()
            .expect("RwLock should never be poisoned. This is a bug.")
            .replace(disambiguation);
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            negotiated_version: RwLock::new(None),
            connection: RwLock::new(Connection::default()),
            disambiguation: RwLock::new(None),
        }
    }
}
