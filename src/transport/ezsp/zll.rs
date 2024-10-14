use crate::ember::key::Data;
use crate::ember::zll::{InitialSecurityState, Network};
use crate::ezsp::zll::NetworkOperation;
use crate::frame::parameters::zll::{network_ops, set_initial_security_state};
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

    /// This call will cause the device to setup the security information used in its network.
    ///
    /// It must be called prior to forming, starting, or joining a network.
    fn set_initial_security_state(
        &mut self,
        network_key: Data,
        security_state: InitialSecurityState,
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

    async fn set_initial_security_state(
        &mut self,
        network_key: Data,
        security_state: InitialSecurityState,
    ) -> Result<(), Error> {
        self.communicate::<_, set_initial_security_state::Response>(
            set_initial_security_state::Command::new(network_key, security_state),
        )
        .await?
        .resolve()
    }
}
