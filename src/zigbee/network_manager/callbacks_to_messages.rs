use log::error;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::Callback;
use crate::zigbee::network_manager::event_mux::Message;

/// Translate callbacks to messages.
pub async fn callbacks_to_messages(mut callbacks: Receiver<Callback>, messages: Sender<Message>) {
    while let Some(callback) = callbacks.recv().await {
        if let Err(error) = messages.send(callback.into()).await {
            error!("Callback to message translation channel is congested: {error}");
        }
    }
}
