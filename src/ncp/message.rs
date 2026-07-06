use tokio::sync::oneshot::Sender;

use crate::Callback;
use crate::ember::Status;
use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};

/// Messages exchanged with the message handler.
#[derive(Debug)]
pub enum Message {
    /// An incoming callback.
    Callback(Box<Callback>),

    /// A request to scan networks.
    NetworkScan(Sender<Vec<NetworkFound>>),

    /// A request to scan channels.
    ChannelScan(Sender<Vec<EnergyScanResult>>),

    /// Register message sent
    Sent {
        /// The message tag.
        tag: u8,
        /// The result of the sending from the stack.
        sender: Sender<Result<Status, u8>>,
    },

    /// Termination signal.
    Terminate,
}

impl From<Box<Callback>> for Message {
    fn from(callback: Box<Callback>) -> Self {
        Self::Callback(callback)
    }
}

impl From<Callback> for Message {
    fn from(callback: Callback) -> Self {
        Self::from(Box::new(callback))
    }
}

impl From<Sender<Vec<NetworkFound>>> for Message {
    fn from(sender: Sender<Vec<NetworkFound>>) -> Self {
        Self::NetworkScan(sender)
    }
}

impl From<Sender<Vec<EnergyScanResult>>> for Message {
    fn from(sender: Sender<Vec<EnergyScanResult>>) -> Self {
        Self::ChannelScan(sender)
    }
}
