//! WWAH Frames

pub mod get_parent_classification_enabled;
pub mod is_hub_connected;
pub mod is_uptime_long;
pub mod set_hub_connectivity;
pub mod set_long_uptime;
pub mod set_parent_classification_enabled;

/// Works With All Hosts (`WWAH`) response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Get parent classification enabled response parameters.
    GetParentClassificationEnabled(get_parent_classification_enabled::Response),
    /// Is hub connected response parameters.
    IsHubConnected(is_hub_connected::Response),
    /// Is uptime long response parameters.
    IsUptimeLong(is_uptime_long::Response),
    /// Set hub connectivity response parameters.
    SetHubConnectivity(set_hub_connectivity::Response),
    /// Set long uptime response parameters.
    SetLongUptime(set_long_uptime::Response),
    /// Set parent classification enabled response parameters.
    SetParentClassificationEnabled(set_parent_classification_enabled::Response),
}
