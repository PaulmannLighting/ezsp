use crate::ember::key::Data;
use crate::ember::node::Type;
use crate::ember::radio::PowerMode;
use crate::ember::zll::{DataToken, InitialSecurityState, Network};
use crate::ezsp::zll::NetworkOperation;
use crate::frame::parameters::zll::{
    get_tokens, is_zll_network, network_ops, set_data_token, set_initial_security_state,
    set_non_zll_network, set_radio_idle_mode, set_rx_on_when_idle, set_security_state_without_key,
    start_scan,
};
use crate::resolve::Resolve;
use crate::{Error, Transport};
use std::future::Future;

/// The `Zll` trait provides an interface for the Zigbee Light Link (ZLL) protocol.
pub trait Zll {
    /// Get the ZLL tokens.
    fn get_tokens(&mut self) -> impl Future<Output = Result<get_tokens::Response, Error>> + Send;

    /// Is this a ZLL network?
    #[allow(clippy::wrong_self_convention)]
    fn is_zll_network(&mut self) -> impl Future<Output = Result<bool, Error>> + Send;

    /// A consolidation of ZLL network operations with similar signatures;
    /// specifically, forming and joining networks or touch-linking.
    fn network_ops(
        &mut self,
        network_info: Network,
        op: NetworkOperation,
        radio_tx_power: i8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Set the ZLL data token.
    fn set_data_token(&mut self, data: DataToken)
        -> impl Future<Output = Result<(), Error>> + Send;

    /// This call will cause the device to setup the security information used in its network.
    ///
    /// It must be called prior to forming, starting, or joining a network.
    fn set_initial_security_state(
        &mut self,
        network_key: Data,
        security_state: InitialSecurityState,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Set the ZLL data token bitmask to reflect the ZLL network state.
    fn set_non_zll_network(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// This call sets the radio's default idle power mode.
    fn set_radio_idle_mode(
        &mut self,
        mode: PowerMode,
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
    async fn get_tokens(&mut self) -> Result<get_tokens::Response, Error> {
        self.communicate::<_, get_tokens::Response>(get_tokens::Command)
            .await
    }

    async fn is_zll_network(&mut self) -> Result<bool, Error> {
        self.communicate::<_, is_zll_network::Response>(is_zll_network::Command)
            .await
            .map(|response| response.is_zll_network())
    }

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

    async fn set_data_token(&mut self, data: DataToken) -> Result<(), Error> {
        self.communicate::<_, set_data_token::Response>(set_data_token::Command::new(data))
            .await
            .map(drop)
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

    async fn set_non_zll_network(&mut self) -> Result<(), Error> {
        self.communicate::<_, set_non_zll_network::Response>(set_non_zll_network::Command)
            .await
            .map(drop)
    }

    async fn set_radio_idle_mode(&mut self, mode: PowerMode) -> Result<(), Error> {
        self.communicate::<_, set_radio_idle_mode::Response>(set_radio_idle_mode::Command::new(
            mode,
        ))
        .await
        .map(drop)
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
