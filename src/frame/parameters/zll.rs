//! ZLL Frames

pub mod clear_tokens;
pub mod get_primary_channel_mask;
pub mod get_secondary_channel_mask;
pub mod get_tokens;
pub mod handler;
pub mod is_zll_network;
pub mod network_ops;
pub mod operation_in_progress;
pub mod rx_on_when_idle_get_active;
pub mod set_additional_state;
pub mod set_data_token;
pub mod set_initial_security_state;
pub mod set_node_type;
pub mod set_non_zll_network;
pub mod set_primary_channel_mask;
pub mod set_radio_idle_mode;
pub mod set_rx_on_when_idle;
pub mod set_secondary_channel_mask;
pub mod set_security_state_without_key;
pub mod start_scan;

/// ZLL response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Clear tokens response parameters.
    ClearTokens(clear_tokens::Response),
    /// Get primary channel mask response parameters.
    GetPrimaryChannelMask(get_primary_channel_mask::Response),
    /// Get secondary channel mask response parameters.
    GetSecondaryChannelMask(get_secondary_channel_mask::Response),
    /// Get tokens response parameters.
    GetTokens(get_tokens::Response),
    /// Is ZLL network response parameters.
    IsZllNetwork(is_zll_network::Response),
    /// Network operations response parameters.
    NetworkOps(network_ops::Response),
    /// Operation in progress response parameters.
    OperationInProgress(operation_in_progress::Response),
    /// RX on when idle get active response parameters.
    RxOnWhenIdleGetActive(rx_on_when_idle_get_active::Response),
    /// Set additional state response parameters.
    SetAdditionalState(set_additional_state::Response),
    /// Set data token response parameters.
    SetDataToken(set_data_token::Response),
    /// Set initial security state response parameters.
    SetInitialSecurityState(set_initial_security_state::Response),
    /// Set node type response parameters.
    SetNodeType(set_node_type::Response),
    /// Set non-ZLL network response parameters.
    SetNonZllNetwork(set_non_zll_network::Response),
    /// Set primary channel mask response parameters.
    SetPrimaryChannelMask(set_primary_channel_mask::Response),
    /// Set radio idle mode response parameters.
    SetRadioIdleMode(set_radio_idle_mode::Response),
    /// Set RX on when idle response parameters.
    SetRxOnWhenIdle(set_rx_on_when_idle::Response),
    /// Set secondary channel mask response parameters.
    SetSecondaryChannelMask(set_secondary_channel_mask::Response),
    /// Set security state without key response parameters.
    SetSecurityStateWithoutKey(set_security_state_without_key::Response),
    /// Start scan response parameters.
    StartScan(start_scan::Response),
}
