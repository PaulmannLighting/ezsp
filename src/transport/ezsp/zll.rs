use crate::ember::key::Data;
use crate::ember::node::Type;
use crate::ember::zll::{InitialSecurityState, Network};
use crate::ezsp::zll::NetworkOperation;
use crate::frame::parameters::zll::{
    network_ops, set_initial_security_state, set_rx_on_when_idle, set_security_state_without_key,
    start_scan,
};
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

    /// This call will change the mode of the radio so that the receiver is on for a specified
    /// amount of time when the device is idle.
    fn set_rx_on_when_idle(
        &mut self,
        duration_millis: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This call will update ZLL security token information.
    ///
    /// Unlike emberZllSetInitialSecurityState, this can be called while a network
    /// is already established.
    fn set_security_state_without_key(
        &mut self,
        security_state: InitialSecurityState,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This call will initiate a ZLL network scan on all the specified channels.
    fn start_scan(
        &mut self,
        channel_mask: u32,
        radio_power_for_scan: i8,
        node_type: Type,
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

    async fn set_rx_on_when_idle(&mut self, duration_millis: u32) -> Result<(), Error> {
        self.communicate::<_, set_rx_on_when_idle::Response>(set_rx_on_when_idle::Command::new(
            duration_millis,
        ))
        .await?
        .resolve()
    }

    async fn set_security_state_without_key(
        &mut self,
        security_state: InitialSecurityState,
    ) -> Result<(), Error> {
        self.communicate::<_, set_security_state_without_key::Response>(
            set_security_state_without_key::Command::new(security_state),
        )
        .await?
        .resolve()
    }

    async fn start_scan(
        &mut self,
        channel_mask: u32,
        radio_power_for_scan: i8,
        node_type: Type,
    ) -> Result<(), Error> {
        self.communicate::<_, start_scan::Response>(start_scan::Command::new(
            channel_mask,
            radio_power_for_scan,
            node_type,
        ))
        .await?
        .resolve()
    }
}
