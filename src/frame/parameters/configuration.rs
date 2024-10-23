//! Configuration Frames

pub(crate) mod add_endpoint;
pub(crate) mod get_configuration_value;
pub(crate) mod get_extended_value;
pub(crate) mod get_policy;
pub(crate) mod get_value;
pub mod read_attribute;
pub(crate) mod send_pan_id_update;
pub(crate) mod set_configuration_value;
pub(crate) mod set_passive_ack_config;
pub(crate) mod set_policy;
pub(crate) mod set_value;
pub mod version;
pub mod write_attribute;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    AddEndpoint(add_endpoint::Command),
    GetConfigurationValue(get_configuration_value::Command),
    GetExtendedValue(get_extended_value::Command),
    GetPolicy(get_policy::Command),
    GetValue(get_value::Command),
    ReadAttribute(read_attribute::Command),
    SendPanIdUpdate(send_pan_id_update::Command),
    SetConfigurationValue(set_configuration_value::Command),
    SetPassiveAckConfig(set_passive_ack_config::Command),
    SetPolicy(set_policy::Command),
    SetValue(set_value::Command),
    Version(version::Command),
    WriteAttribute(write_attribute::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    AddEndpoint(add_endpoint::Response),
    GetConfigurationValue(get_configuration_value::Response),
    GetExtendedValue(get_extended_value::Response),
    GetPolicy(get_policy::Response),
    GetValue(get_value::Response),
    ReadAttribute(read_attribute::Response),
    SendPanIdUpdate(send_pan_id_update::Response),
    SetConfigurationValue(set_configuration_value::Response),
    SetPassiveAckConfig(set_passive_ack_config::Response),
    SetPolicy(set_policy::Response),
    SetValue(set_value::Response),
    Version(version::Response),
    WriteAttribute(write_attribute::Response),
}
