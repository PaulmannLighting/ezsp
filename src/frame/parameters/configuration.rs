//! Configuration Frames

pub mod add_endpoint;
pub mod get_configuration_value;
pub mod get_extended_value;
pub mod get_policy;
pub mod get_value;
pub mod read_attribute;
pub mod send_pan_id_update;
pub mod set_configuration_value;
pub mod set_passive_ack_config;
pub mod set_policy;
pub mod set_value;
pub mod version;
pub mod write_attribute;

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
