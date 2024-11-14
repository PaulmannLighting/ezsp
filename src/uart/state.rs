use std::fmt::Debug;

use log::trace;

use crate::frame::Disambiguation;
use crate::transport::MIN_NON_LEGACY_VERSION;
use crate::uart::connection::Connection;

/// Shared state of the `EZSP` UART.
#[derive(Debug, Default)]
pub struct State {
    negotiated_version: Option<u8>,
    connection: Connection,
    disambiguation: Option<Disambiguation>,
}

impl State {
    /// Return the negotiated version.
    ///
    /// Returns `None` if the version has not been negotiated yet.
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn negotiated_version(&self) -> Option<u8> {
        self.negotiated_version
    }

    /// Set the negotiated version.
    pub fn set_negotiated_version(&mut self, version: u8) {
        self.negotiated_version.replace(version);
    }

    /// Returns the connection state of the UART.
    #[must_use]
    pub fn connection(&self) -> Connection {
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
            .map_or(true, |version| version < MIN_NON_LEGACY_VERSION)
    }

    /// Returns the disambiguation.
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn disambiguation(&self) -> Option<Disambiguation> {
        self.disambiguation
    }

    /// Set the disambiguation.
    pub fn set_disambiguation(&mut self, disambiguation: Disambiguation) {
        self.disambiguation.replace(disambiguation);
    }

    /// Returns `true` if a response is pending else `false`.
    pub fn is_response_pending(&self) -> bool {
        self.disambiguation().is_some()
    }
}
