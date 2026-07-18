//! Shared ownership of an EZSP transport.

use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard};

use crate::Transport;

/// An asynchronously locked, reference-counted EZSP transport.
///
/// Clones refer to the same underlying transport. Callers must hold the lock
/// for an entire EZSP command/response transaction, then release it before
/// waiting for asynchronous callbacks.
#[derive(Debug)]
pub struct SharedTransport<T>
where
    T: Transport,
{
    inner: Arc<Mutex<T>>,
}

impl<T> SharedTransport<T>
where
    T: Transport,
{
    /// Wraps a transport for shared ownership.
    #[must_use]
    pub fn new(transport: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(transport)),
        }
    }

    /// Locks the underlying transport for an EZSP transaction.
    pub async fn lock(&self) -> MutexGuard<'_, T> {
        self.inner.lock().await
    }
}

impl<T> Clone for SharedTransport<T>
where
    T: Transport,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> From<T> for SharedTransport<T>
where
    T: Transport,
{
    fn from(transport: T) -> Self {
        Self::new(transport)
    }
}
