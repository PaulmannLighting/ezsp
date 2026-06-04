use tokio::sync::oneshot;
use zigbee_hw::{FoundNetwork, ScannedChannel};

use crate::Callback;

/// Messages exchanged with the message handler.
#[derive(Debug)]
pub enum Message {
    /// An incoming callback.
    Callback(Box<Callback>),
    /// A request to scan networks.
    NetworkScan(oneshot::Sender<Vec<FoundNetwork>>),
    /// A request to scan channels.
    ChannelScan(oneshot::Sender<Vec<ScannedChannel>>),
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

impl From<oneshot::Sender<Vec<FoundNetwork>>> for Message {
    fn from(sender: oneshot::Sender<Vec<FoundNetwork>>) -> Self {
        Self::NetworkScan(sender)
    }
}

impl From<oneshot::Sender<Vec<ScannedChannel>>> for Message {
    fn from(sender: oneshot::Sender<Vec<ScannedChannel>>) -> Self {
        Self::ChannelScan(sender)
    }
}
