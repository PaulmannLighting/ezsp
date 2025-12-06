use tokio::sync::mpsc::Receiver;
use zigbee_nwk::FoundNetwork;

use crate::parameters::networking::handler::Handler as Networking;
use crate::{Callback, Error};

/// Trait for collecting found networks from a callback receiver.
pub trait CollectNetworksFound {
    /// Collect found networks into a vector.
    fn collect_networks_found(
        &mut self,
    ) -> impl Future<Output = Result<Vec<FoundNetwork>, Error>> + Send;
}

impl CollectNetworksFound for Receiver<Callback> {
    async fn collect_networks_found(&mut self) -> Result<Vec<FoundNetwork>, Error> {
        let mut networks = Vec::new();

        while let Some(callback) = self.recv().await {
            if let Callback::Networking(Networking::NetworkFound(network_found)) = callback {
                networks.push(network_found.into());
            }
        }

        Ok(networks)
    }
}
