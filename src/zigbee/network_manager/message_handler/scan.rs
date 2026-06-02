use tokio::sync::oneshot::Sender;
use zigbee_hw::{FoundNetwork, ScannedChannel};

#[derive(Debug)]
pub enum Scan {
    /// A channel scan.
    Channel(Sender<Vec<ScannedChannel>>),
    /// A network scan.
    Network(Sender<Vec<FoundNetwork>>),
}

impl From<Sender<Vec<ScannedChannel>>> for Scan {
    fn from(sender: Sender<Vec<ScannedChannel>>) -> Self {
        Self::Channel(sender)
    }
}

impl From<Sender<Vec<FoundNetwork>>> for Scan {
    fn from(sender: Sender<Vec<FoundNetwork>>) -> Self {
        Self::Network(sender)
    }
}
