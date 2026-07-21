//! ZLL Frames

pub use self::clear_tokens::Response as ClearTokens;
pub use self::get_primary_channel_mask::Response as GetPrimaryChannelMask;
pub use self::get_secondary_channel_mask::Response as GetSecondaryChannelMask;
pub use self::get_tokens::Response as GetTokens;
pub use self::is_zll_network::Response as IsZllNetwork;
pub use self::network_ops::Response as NetworkOps;
pub use self::operation_in_progress::Response as OperationInProgress;
pub use self::rx_on_when_idle_get_active::Response as RxOnWhenIdleGetActive;
pub use self::set_additional_state::Response as SetAdditionalState;
pub use self::set_data_token::Response as SetDataToken;
pub use self::set_initial_security_state::Response as SetInitialSecurityState;
pub use self::set_node_type::Response as SetNodeType;
pub use self::set_non_zll_network::Response as SetNonZllNetwork;
pub use self::set_primary_channel_mask::Response as SetPrimaryChannelMask;
pub use self::set_radio_idle_mode::Response as SetRadioIdleMode;
pub use self::set_rx_on_when_idle::Response as SetRxOnWhenIdle;
pub use self::set_secondary_channel_mask::Response as SetSecondaryChannelMask;
pub use self::set_security_state_without_key::Response as SetSecurityStateWithoutKey;
pub use self::start_scan::Response as StartScan;

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

crate::frame::parameters::command_enum!(
    Command,
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
);

crate::frame::parameters::parameter_enum!(
    Response,
    ClearTokens,
    GetPrimaryChannelMask,
    GetSecondaryChannelMask,
    GetTokens,
    IsZllNetwork,
    NetworkOps,
    OperationInProgress,
    RxOnWhenIdleGetActive,
    SetAdditionalState,
    SetDataToken,
    SetInitialSecurityState,
    SetNodeType,
    SetNonZllNetwork,
    SetPrimaryChannelMask,
    SetRadioIdleMode,
    SetRxOnWhenIdle,
    SetSecondaryChannelMask,
    SetSecurityStateWithoutKey,
    StartScan
);
