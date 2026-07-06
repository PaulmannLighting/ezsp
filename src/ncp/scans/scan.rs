use tokio::sync::oneshot::Sender;

use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};

/// Types of scans that may be completed by a `ScanComplete` handler.
#[derive(Debug)]
pub enum Scan {
    /// A channel scan.
    Channel(Sender<Vec<EnergyScanResult>>),

    /// A network scan.
    Network(Sender<Vec<NetworkFound>>),
}

impl From<Sender<Vec<EnergyScanResult>>> for Scan {
    fn from(sender: Sender<Vec<EnergyScanResult>>) -> Self {
        Self::Channel(sender)
    }
}

impl From<Sender<Vec<NetworkFound>>> for Scan {
    fn from(sender: Sender<Vec<NetworkFound>>) -> Self {
        Self::Network(sender)
    }
}
