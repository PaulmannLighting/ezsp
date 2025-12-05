use std::sync::Arc;

use log::{debug, trace, warn};
use tokio::sync::Mutex;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::task::{JoinError, JoinHandle};

use crate::Callback;

type Handlers = Arc<Mutex<Vec<Sender<Callback>>>>;

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
    pub async fn register(&self, buffer: usize) -> Receiver<Callback> {
        let (sender, receiver) = channel(buffer);
        self.handlers.lock().await.push(sender);
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
    while let Some(event) = events.recv().await {
        trace!("Received EZSP event: {event:?}");
        let mut lock = handlers.lock().await;
        lock.retain(|sender| !sender.is_closed());

        for handler in lock.iter() {
            trace!("Forwarding event to handler: {handler:?}");

            if let Err(error) = handler.try_send(event.clone()) {
                match error {
                    TrySendError::Full(_) => {
                        warn!("Event handler is congested: {handler:?}");
                    }
                    TrySendError::Closed(_) => {
                        trace!("Event handler has been closed: {handler:?}");
                    }
                }
            }
        }
    }

    debug!("EZSP event forwarder has stopped receiving events.");
}
