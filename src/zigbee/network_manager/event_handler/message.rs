use tokio::sync::oneshot::Sender;
use zigbee_hw::{FoundNetwork, ScannedChannel};

use crate::Callback;
use crate::ember::Status;

/// Messages exchanged with the message handler.
#[derive(Debug)]
pub enum Message {
    /// An incoming callback.
    Callback(Box<Callback>),

    /// A request to scan networks.
    NetworkScan(Sender<Vec<FoundNetwork>>),

    /// A request to scan channels.
    ChannelScan(Sender<Vec<ScannedChannel>>),

    /// Register message sent
    MessageSent {
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

impl From<Sender<Vec<FoundNetwork>>> for Message {
    fn from(sender: Sender<Vec<FoundNetwork>>) -> Self {
        Self::NetworkScan(sender)
    }
}

impl From<Sender<Vec<ScannedChannel>>> for Message {
    fn from(sender: Sender<Vec<ScannedChannel>>) -> Self {
        Self::ChannelScan(sender)
    }
}
