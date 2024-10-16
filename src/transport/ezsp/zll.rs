use crate::ember::key::Data;
use crate::ember::node::Type;
use crate::ember::radio::PowerMode;
use crate::ember::zll::{DataToken, InitialSecurityState, Network};
use crate::ezsp::zll::NetworkOperation;
use crate::frame::parameters::zll::{
    clear_tokens, get_primary_channel_mask, get_secondary_channel_mask, get_tokens, is_zll_network,
    network_ops, operation_in_progress, rx_on_when_idle_get_active, set_additional_state,
    set_data_token, set_initial_security_state, set_node_type, set_non_zll_network,
    set_primary_channel_mask, set_radio_idle_mode, set_rx_on_when_idle, set_secondary_channel_mask,
    set_security_state_without_key, start_scan,
};
use crate::resolve::Resolve;
use crate::{Error, Transport};
use std::future::Future;

/// The `Zll` trait provides an interface for the Zigbee Light Link (ZLL) protocol.
pub trait Zll {
    /// Clear ZLL stack tokens.
    fn clear_tokens(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Get the primary ZLL (touchlink) channel mask.
    fn get_primary_channel_mask(&mut self) -> impl Future<Output = Result<u32, Error>> + Send;

    /// Get the secondary ZLL (touchlink) channel mask.
    fn get_secondary_channel_mask(&mut self) -> impl Future<Output = Result<u32, Error>> + Send;

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

    /// Is there a ZLL (Touchlink) operation in progress?
    fn operation_in_progress(&mut self) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Is the ZLL radio on when idle mode is active?
    fn rx_on_when_idle_get_active(&mut self) -> impl Future<Output = Result<bool, Error>> + Send;

    /// This call sets additional capability bits in the ZLL state.
    fn set_additional_state(
        &mut self,
        state: u16,
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

    /// This call sets the default node type for a factory new ZLL device.
    fn set_node_type(&mut self, node_type: Type) -> impl Future<Output = Result<(), Error>> + Send;

    /// Set the ZLL data token bitmask to reflect the ZLL network state.
    fn set_non_zll_network(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Set the primary ZLL (touchlink) channel mask
    fn set_primary_channel_mask(
        &mut self,
        mask: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;

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

    /// Set the secondary ZLL (touchlink) channel mask.
    fn set_secondary_channel_mask(
        &mut self,
        mask: u32,
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
    async fn clear_tokens(&mut self) -> Result<(), Error> {
        self.communicate::<_, clear_tokens::Response>(clear_tokens::Command)
            .await
            .map(drop)
    }

    async fn get_primary_channel_mask(&mut self) -> Result<u32, Error> {
        self.communicate::<_, get_primary_channel_mask::Response>(get_primary_channel_mask::Command)
            .await
            .map(|response| response.zll_primary_channel_mask())
    }

    async fn get_secondary_channel_mask(&mut self) -> Result<u32, Error> {
        self.communicate::<_, get_secondary_channel_mask::Response>(
            get_secondary_channel_mask::Command,
        )
        .await
        .map(|response| response.zll_secondary_channel_mask())
    }

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

    async fn operation_in_progress(&mut self) -> Result<bool, Error> {
        self.communicate::<_, operation_in_progress::Response>(operation_in_progress::Command)
            .await
            .map(|response| response.zll_operation_in_progress())
    }

    async fn rx_on_when_idle_get_active(&mut self) -> Result<bool, Error> {
        self.communicate::<_, rx_on_when_idle_get_active::Response>(
            rx_on_when_idle_get_active::Command,
        )
        .await
        .map(|response| response.zll_rx_on_when_idle_get_active())
    }
    async fn set_additional_state(&mut self, state: u16) -> Result<(), Error> {
        self.communicate::<_, set_additional_state::Response>(set_additional_state::Command::new(
            state,
        ))
        .await
        .map(drop)
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

    async fn set_node_type(&mut self, node_type: Type) -> Result<(), Error> {
        self.communicate::<_, set_node_type::Response>(set_node_type::Command::new(node_type))
            .await
            .map(drop)
    }

    async fn set_non_zll_network(&mut self) -> Result<(), Error> {
        self.communicate::<_, set_non_zll_network::Response>(set_non_zll_network::Command)
            .await
            .map(drop)
    }

    async fn set_primary_channel_mask(&mut self, mask: u32) -> Result<(), Error> {
        self.communicate::<_, set_primary_channel_mask::Response>(
            set_primary_channel_mask::Command::new(mask),
        )
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

    async fn set_secondary_channel_mask(&mut self, mask: u32) -> Result<(), Error> {
        self.communicate::<_, set_secondary_channel_mask::Response>(
            set_secondary_channel_mask::Command::new(mask),
        )
        .await
        .map(drop)
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
