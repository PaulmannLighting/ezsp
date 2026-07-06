use tokio::sync::oneshot::Sender;

use crate::parameters::networking::handler::{EnergyScanResult, NetworkFound};

/// Pending scan result type completed by a `scanComplete` callback.
#[derive(Debug)]
pub enum Scan {
    /// Energy scan result stream.
    Channel(Sender<Vec<EnergyScanResult>>),

    /// Active network scan result stream.
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
