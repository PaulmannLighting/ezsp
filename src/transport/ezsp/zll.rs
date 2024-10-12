use crate::ember::zll::Network;
use crate::ezsp::zll::NetworkOperation;
use crate::frame::parameters::zll::network_ops;
use crate::resolve::Resolve;
use crate::{Error, Transport};
use std::future::Future;

/// The `Zll` trait provides an interface for the Zigbee Light Link (ZLL) protocol.
pub trait Zll {
    /// A consolidation of ZLL network operations with similar signatures;
    /// specifically, forming and joining networks or touch-linking.
    fn network_ops(
        &mut self,
        network_info: Network,
        op: NetworkOperation,
        radio_tx_power: i8,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Zll for T
where
    T: Transport,
{
    async fn network_ops(
        &mut self,
        network_info: Network,
        op: NetworkOperation,
        radio_tx_power: i8,
    ) -> Result<(), Error> {
        self.communicate::<_, network_ops::Response>(network_ops::Command::new(
            network_info,
            op,
            radio_tx_power,
        ))
        .await?
        .resolve()
    }
}
