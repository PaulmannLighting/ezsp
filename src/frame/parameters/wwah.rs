//! WWAH Frames

pub use self::get_parent_classification_enabled::Response as GetParentClassificationEnabled;
pub use self::is_hub_connected::Response as IsHubConnected;
pub use self::is_uptime_long::Response as IsUptimeLong;
pub use self::set_hub_connectivity::Response as SetHubConnectivity;
pub use self::set_long_uptime::Response as SetLongUptime;
pub use self::set_parent_classification_enabled::Response as SetParentClassificationEnabled;

pub mod get_parent_classification_enabled;
pub mod is_hub_connected;
pub mod is_uptime_long;
pub mod set_hub_connectivity;
pub mod set_long_uptime;
pub mod set_parent_classification_enabled;

crate::frame::parameters::command_enum!(
    Command,
    GetParentClassificationEnabled(get_parent_classification_enabled::Command),
    IsHubConnected(is_hub_connected::Command),
    IsUptimeLong(is_uptime_long::Command),
    SetHubConnectivity(set_hub_connectivity::Command),
    SetLongUptime(set_long_uptime::Command),
    SetParentClassificationEnabled(set_parent_classification_enabled::Command),
);

crate::frame::parameters::parameter_enum!(
    Response,
    GetParentClassificationEnabled,
    IsHubConnected,
    IsUptimeLong,
    SetHubConnectivity,
    SetLongUptime,
    SetParentClassificationEnabled
);
