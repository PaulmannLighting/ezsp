use std::io;

use tokio::sync::mpsc::Receiver;
use zigbee_nwk::{FoundNetwork, ScannedChannel};

use crate::parameters::networking::handler::Handler as Networking;
use crate::{Callback, Error};

/// Trait for collecting found networks from a callback receiver.
pub trait CollectNetworksFound {
    /// Collect found networks into a vector.
    fn collect_networks_found(
        &mut self,
    ) -> impl Future<Output = Result<Vec<FoundNetwork>, Error>> + Send;

    /// Collect found channels into a vector.
    fn collect_scanned_channels(
        &mut self,
    ) -> impl Future<Output = Result<Vec<ScannedChannel>, Error>> + Send;
}

impl CollectNetworksFound for Receiver<Callback> {
    async fn collect_networks_found(&mut self) -> Result<Vec<FoundNetwork>, Error> {
        let mut networks = Vec::new();

        while let Some(callback) = self.recv().await {
            if let Callback::Networking(Networking::NetworkFound(network_found)) = callback {
                networks.push(network_found.into());
            } else if let Callback::Networking(Networking::ScanComplete(scan_complete)) = callback {
                return scan_complete.status().map(|()| networks);
            }
        }

        Err(Error::Io(io::Error::other(
            "Callback channel closed before scan completed",
        )))
    }

    async fn collect_scanned_channels(&mut self) -> Result<Vec<ScannedChannel>, Error> {
        let mut networks = Vec::new();

        while let Some(callback) = self.recv().await {
            if let Callback::Networking(Networking::EnergyScanResult(energy_scan_result)) = callback
            {
                networks.push(energy_scan_result.into());
            } else if let Callback::Networking(Networking::ScanComplete(scan_complete)) = callback {
                return scan_complete.status().map(|()| networks);
            }
        }

        Err(Error::Io(io::Error::other(
            "Callback channel closed before scan completed",
        )))
    }
}
