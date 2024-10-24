//! WWAH Frames

pub mod get_parent_classification_enabled;
pub mod is_hub_connected;
pub mod is_uptime_long;
pub mod set_hub_connectivity;
pub mod set_long_uptime;
pub mod set_parent_classification_enabled;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    GetParentClassificationEnabled(get_parent_classification_enabled::Response),
    IsHubConnected(is_hub_connected::Response),
    IsUptimeLong(is_uptime_long::Response),
    SetHubConnectivity(set_hub_connectivity::Response),
    SetLongUptime(set_long_uptime::Response),
    SetParentClassificationEnabled(set_parent_classification_enabled::Response),
}
