use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, RwLock};

use crate::frame::Disambiguation;
use crate::transport::MIN_NON_LEGACY_VERSION;
use crate::uart::connection::Connection;
use log::trace;

/// Shared state of the `EZSP` UART.
#[derive(Clone, Debug)]
pub struct State {
    negotiated_version: Arc<RwLock<Option<u8>>>,
    connection: Arc<RwLock<Connection>>,
    pending_requests: Arc<AtomicUsize>,
    disambiguation: Arc<RwLock<Option<Disambiguation>>>,
}

impl State {
    /// Return the negotiated version.
    ///
    /// Returns `None` if the version has not been negotiated yet.
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn negotiated_version(&self) -> Option<u8> {
        *self.negotiated_version.read().expect("RwLock poisoned")
    }

    /// Set the negotiated version.
    pub fn set_negotiated_version(&self, version: u8) {
        self.negotiated_version
            .write()
            .expect("RwLock poisoned")
            .replace(version);
    }

    /// Returns the connection state of the UART.
    #[must_use]
    pub fn connection(&self) -> Connection {
        *self.connection.read().expect("RwLock poisoned")
    }

    /// Set the connection state of the UART.
    pub fn set_connection(&self, connection: Connection) {
        trace!("Setting connection state to: {connection:?}");
        *self.connection.write().expect("RwLock poisoned") = connection;

        if connection != Connection::Connected {
            trace!("Resetting negotiated version.");
            self.negotiated_version
                .write()
                .expect("RwLock poisoned")
                .take();
        }
    }

    /// Returns `true` if the negotiated version is a legacy version.
    #[must_use]
    pub fn is_legacy(&self) -> bool {
        self.negotiated_version()
            .map_or(true, |version| version < MIN_NON_LEGACY_VERSION)
    }

    /// Returns whether requests are pending.
    #[must_use]
    pub fn requests_pending(&self) -> bool {
        self.pending_requests.load(Relaxed) > 0
    }

    /// Returns the disambiguation.
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn disambiguation(&self) -> Option<Disambiguation> {
        *self.disambiguation.read().expect("RwLock poisoned")
    }

    /// Set the disambiguation.
    pub fn set_disambiguation(&self, disambiguation: Option<Disambiguation>) {
        *self.disambiguation.write().expect("RwLock poisoned") = disambiguation;
    }

    /// Increment the number of pending requests.
    pub fn increment_requests(&self) {
        self.pending_requests.fetch_add(1, Relaxed);
    }

    /// Decrement the number of pending requests.
    pub fn decrement_requests(&self) {
        self.pending_requests.fetch_sub(1, Relaxed);
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            negotiated_version: Arc::new(RwLock::new(None)),
            connection: Arc::new(RwLock::new(Connection::default())),
            pending_requests: Arc::new(AtomicUsize::new(0)),
            disambiguation: Arc::new(RwLock::new(None)),
        }
    }
}
