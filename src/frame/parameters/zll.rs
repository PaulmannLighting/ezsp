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
