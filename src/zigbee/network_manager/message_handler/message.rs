use tokio::sync::mpsc::Sender;
use zigbee_hw::Event;

use crate::Callback;

/// Messages exchanged with the message handler.
#[derive(Clone, Debug)]
pub enum Message {
    /// An incoming callback.
    Callback(Callback),
    /// An incoming subscription request.
    Subscribe(Sender<Event>),
}

impl From<Callback> for Message {
    fn from(callback: Callback) -> Self {
        Self::Callback(callback)
    }
}

impl From<Sender<Event>> for Message {
    fn from(sender: Sender<Event>) -> Self {
        Self::Subscribe(sender)
    }
}
