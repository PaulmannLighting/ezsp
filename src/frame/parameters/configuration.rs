//! Configuration parameters.

pub use self::add_endpoint::Response as AddEndpoint;
pub use self::get_configuration_value::Response as GetConfigurationValue;
pub use self::get_extended_value::Response as GetExtendedValue;
pub use self::get_policy::Response as GetPolicy;
pub use self::get_value::Response as GetValue;
pub use self::read_attribute::Response as ReadAttribute;
pub use self::send_pan_id_update::Response as SendPanIdUpdate;
pub use self::set_configuration_value::Response as SetConfigurationValue;
pub use self::set_passive_ack_config::Response as SetPassiveAckConfig;
pub use self::set_policy::Response as SetPolicy;
pub use self::set_value::Response as SetValue;
pub use self::version::Response as Version;
pub use self::write_attribute::Response as WriteAttribute;

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

crate::frame::parameters::command_enum!(
    Command,
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
);

crate::frame::parameters::parameter_enum!(
    Response,
    AddEndpoint,
    GetConfigurationValue,
    GetExtendedValue,
    GetPolicy,
    GetValue,
    ReadAttribute,
    SendPanIdUpdate,
    SetConfigurationValue,
    SetPassiveAckConfig,
    SetPolicy,
    SetValue,
    Version,
    WriteAttribute
);
