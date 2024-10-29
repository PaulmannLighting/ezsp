use std::fmt::Debug;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, RwLock};

use crate::transport::MIN_NON_LEGACY_VERSION;

/// Shared state of the UART.
#[derive(Clone, Debug)]
pub struct State {
    negotiated_version: Arc<RwLock<Option<u8>>>,
    needs_reset: Arc<AtomicBool>,
    pending_requests: Arc<AtomicUsize>,
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

    /// Returns `true` if the UART needs a reset.
    ///
    /// This may be due to a lower-level communication error, e.g. on the `ASHv2` level.
    #[must_use]
    pub fn needs_reset(&self) -> bool {
        self.needs_reset.load(Relaxed)
    }

    /// Set the flag that indicates if the UART needs a reset.
    pub fn set_needs_reset(&self, needs_reset: bool) {
        self.needs_reset.store(needs_reset, Relaxed);
    }

    /// Returns `true` if the negotiated version is a legacy version.
    pub fn is_legacy(&self) -> bool {
        self.negotiated_version()
            .map_or(true, |version| version < MIN_NON_LEGACY_VERSION)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            negotiated_version: Arc::new(RwLock::new(None)),
            needs_reset: Arc::new(AtomicBool::new(true)),
            pending_requests: Arc::new(AtomicUsize::new(0)),
        }
    }
}
