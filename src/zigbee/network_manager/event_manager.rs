use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use log::{debug, trace, warn};
use tokio::sync::Mutex;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::task::{JoinError, JoinHandle};
use uuid::Uuid;

pub use self::await_network_up::AwaitNetworkUp;
pub use self::await_not_joined::AwaitNotJoined;
use crate::Callback;

mod await_network_up;
mod await_not_joined;

type Handlers = Arc<Mutex<BTreeMap<Uuid, Sender<Callback>>>>;

/// EZSP event handler.
pub struct EventManager {
    handlers: Handlers,
    task: JoinHandle<()>,
}

impl EventManager {
    /// Create a new event manager.
    pub fn new(events: Receiver<Callback>) -> Self {
        let handlers = Handlers::default();
        let task = tokio::spawn(forward_events(events, handlers.clone()));
        Self { handlers, task }
    }

    /// Register a new event handler.
    pub async fn register(&mut self, buffer: usize) -> Receiver<Callback> {
        let (sender, receiver) = channel(buffer);
        let uuid = Uuid::new_v4();
        debug!("Registering callback handler with UUID: {uuid}");
        self.handlers.lock().await.insert(uuid, sender);
        receiver
    }

    /// Shutdown the event manager.
    ///
    /// # Errors
    ///
    /// Returns an error if the event forwarding task fails to shut down.
    pub async fn shutdown(self) -> Result<(), JoinError> {
        self.task.abort();
        self.task.await
    }
}

async fn forward_events(mut events: Receiver<Callback>, handlers: Handlers) {
    let mut active_handlers = BTreeSet::new();

    while let Some(event) = events.recv().await {
        debug!("Received EZSP event: {event:?}");
        active_handlers.clear();
        let mut lock = handlers.lock().await;

        for (uuid, handler) in lock.iter() {
            match handler.try_send(event.clone()) {
                Ok(()) => {
                    active_handlers.insert(*uuid);
                }
                Err(error) => match error {
                    TrySendError::Full(_) => {
                        warn!("Event handler {uuid} is congested.");
                        active_handlers.insert(*uuid);
                    }
                    TrySendError::Closed(_) => {
                        trace!("Event handler {uuid} has been closed and will be removed.");
                    }
                },
            }
        }

        lock.retain(|uuid, _| active_handlers.contains(uuid));
    }

    debug!("EZSP event forwarder has stopped receiving events.");
}
