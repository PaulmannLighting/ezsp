//! ZLL Frames

pub(crate) mod clear_tokens;
pub(crate) mod get_primary_channel_mask;
pub(crate) mod get_secondary_channel_mask;
pub mod get_tokens;
pub mod handler;
pub(crate) mod is_zll_network;
pub(crate) mod network_ops;
pub(crate) mod operation_in_progress;
pub(crate) mod rx_on_when_idle_get_active;
pub(crate) mod set_additional_state;
pub(crate) mod set_data_token;
pub(crate) mod set_initial_security_state;
pub(crate) mod set_node_type;
pub(crate) mod set_non_zll_network;
pub(crate) mod set_primary_channel_mask;
pub(crate) mod set_radio_idle_mode;
pub(crate) mod set_rx_on_when_idle;
pub(crate) mod set_secondary_channel_mask;
pub(crate) mod set_security_state_without_key;
pub(crate) mod start_scan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    ClearTokens(clear_tokens::Command),
    GetPrimaryChannelMask(get_primary_channel_mask::Command),
    GetSecondaryChannelMask(get_secondary_channel_mask::Command),
    GetTokens(get_tokens::Command),
    IsZllNetwork(is_zll_network::Command),
    NetworkOps(network_ops::Command),
    OperationInProgress(operation_in_progress::Command),
    RxOnWhenIdleGetActive(rx_on_when_idle_get_active::Command),
    SetAdditionalState(set_additional_state::Command),
    SetDataToken(set_data_token::Command),
    SetInitialSecurityState(set_initial_security_state::Command),
    SetNodeType(set_node_type::Command),
    SetNonZllNetwork(set_non_zll_network::Command),
    SetPrimaryChannelMask(set_primary_channel_mask::Command),
    SetRadioIdleMode(set_radio_idle_mode::Command),
    SetRxOnWhenIdle(set_rx_on_when_idle::Command),
    SetSecondaryChannelMask(set_secondary_channel_mask::Command),
    SetSecurityStateWithoutKey(set_security_state_without_key::Command),
    StartScan(start_scan::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    ClearTokens(clear_tokens::Response),
    GetPrimaryChannelMask(get_primary_channel_mask::Response),
    GetSecondaryChannelMask(get_secondary_channel_mask::Response),
    GetTokens(get_tokens::Response),
    IsZllNetwork(is_zll_network::Response),
    NetworkOps(network_ops::Response),
    OperationInProgress(operation_in_progress::Response),
    RxOnWhenIdleGetActive(rx_on_when_idle_get_active::Response),
    SetAdditionalState(set_additional_state::Response),
    SetDataToken(set_data_token::Response),
    SetInitialSecurityState(set_initial_security_state::Response),
    SetNodeType(set_node_type::Response),
    SetNonZllNetwork(set_non_zll_network::Response),
    SetPrimaryChannelMask(set_primary_channel_mask::Response),
    SetRadioIdleMode(set_radio_idle_mode::Response),
    SetRxOnWhenIdle(set_rx_on_when_idle::Response),
    SetSecondaryChannelMask(set_secondary_channel_mask::Response),
    SetSecurityStateWithoutKey(set_security_state_without_key::Response),
    StartScan(start_scan::Response),
    Handler(handler::Handler),
}
