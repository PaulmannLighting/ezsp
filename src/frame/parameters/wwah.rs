//! WWAH Frames

pub(crate) mod get_parent_classification_enabled;
pub(crate) mod is_hub_connected;
pub(crate) mod is_uptime_long;
pub(crate) mod set_hub_connectivity;
pub(crate) mod set_long_uptime;
pub(crate) mod set_parent_classification_enabled;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    GetParentClassificationEnabled(get_parent_classification_enabled::Command),
    IsHubConnected(is_hub_connected::Command),
    IsUptimeLong(is_uptime_long::Command),
    SetHubConnectivity(set_hub_connectivity::Command),
    SetLongUptime(set_long_uptime::Command),
    SetParentClassificationEnabled(set_parent_classification_enabled::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    GetParentClassificationEnabled(get_parent_classification_enabled::Response),
    IsHubConnected(is_hub_connected::Response),
    IsUptimeLong(is_uptime_long::Response),
    SetHubConnectivity(set_hub_connectivity::Response),
    SetLongUptime(set_long_uptime::Response),
    SetParentClassificationEnabled(set_parent_classification_enabled::Response),
}
