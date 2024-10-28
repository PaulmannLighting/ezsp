use log::trace;
use std::future::Future;
use tokio::sync::mpsc::Receiver;

use crate::frame::Callback;

/// A callback handler.
pub trait Handler: Sized + Send + 'static {
    /// Run the handler.
    fn run(mut self, mut callbacks: Receiver<Callback>) -> impl Future<Output = ()> + Send {
        async move {
            trace!("Running handler.");
            while let Some(callback) = callbacks.recv().await {
                trace!("Received callback.");
                self.handle(callback);
            }
        }
    }

    /// Handle a callback.
    fn handle(&mut self, callback: Callback);
}
