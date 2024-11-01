//! Configuration parameters.

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

/// Configuration response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response to the [`add_endpoint`](crate::Configuration::add_endpoint) command.
    AddEndpoint(add_endpoint::Response),
    /// Response to the [`get_configuration_value`](crate::Configuration::get_configuration_value) command.
    GetConfigurationValue(get_configuration_value::Response),
    /// Response to the [`get_extended_value`](crate::Configuration::get_extended_value) command.
    GetExtendedValue(get_extended_value::Response),
    /// Response to the [`get_policy`](crate::Configuration::get_policy) command.
    GetPolicy(get_policy::Response),
    /// Response to the [`get_value`](crate::Configuration::get_value) command.
    GetValue(get_value::Response),
    /// Response to the [`read_attribute`](crate::Configuration::read_attribute) command.
    ReadAttribute(read_attribute::Response),
    /// Response to the [`send_pan_id_update`](crate::Configuration::send_pan_id_update) command.
    SendPanIdUpdate(send_pan_id_update::Response),
    /// Response to the [`set_configuration_value`](crate::Configuration::set_configuration_value) command.
    SetConfigurationValue(set_configuration_value::Response),
    /// Response to the [`set_passive_ack_config`](crate::Configuration::set_passive_ack_config) command.
    SetPassiveAckConfig(set_passive_ack_config::Response),
    /// Response to the [`set_policy`](crate::Configuration::set_policy) command.
    SetPolicy(set_policy::Response),
    /// Response to the [`set_value`](crate::Configuration::set_value) command.
    SetValue(set_value::Response),
    /// Response to the [`version`](crate::Configuration::version) command.
    Version(version::Response),
    /// Response to the [`write_attribute`](crate::Configuration::write_attribute) command.
    WriteAttribute(write_attribute::Response),
}
