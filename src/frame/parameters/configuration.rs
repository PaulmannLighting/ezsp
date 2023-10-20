use crate::read_write::Writable;
use std::io::Write;

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

#[derive(Debug, Eq, PartialEq)]
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
    WriteAttribute(write_attribute::Command),
}

impl Command {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::AddEndpoint(_) => add_endpoint::ID,
            Self::GetConfigurationValue(_) => get_configuration_value::ID,
            Self::GetExtendedValue(_) => get_extended_value::ID,
            Self::GetPolicy(_) => get_policy::ID,
            Self::GetValue(_) => get_value::ID,
            Self::ReadAttribute(_) => read_attribute::ID,
            Self::SendPanIdUpdate(_) => send_pan_id_update::ID,
            Self::SetConfigurationValue(_) => set_configuration_value::ID,
            Self::SetPassiveAckConfig(_) => set_passive_ack_config::ID,
            Self::SetPolicy(_) => set_policy::ID,
            Self::SetValue(_) => set_value::ID,
            Self::WriteAttribute(_) => write_attribute::ID,
        }
    }
}

impl From<add_endpoint::Command> for Command {
    fn from(add_endpoint: add_endpoint::Command) -> Self {
        Self::AddEndpoint(add_endpoint)
    }
}

impl From<get_configuration_value::Command> for Command {
    fn from(get_configuration_value: get_configuration_value::Command) -> Self {
        Self::GetConfigurationValue(get_configuration_value)
    }
}

impl From<get_extended_value::Command> for Command {
    fn from(get_extended_value: get_extended_value::Command) -> Self {
        Self::GetExtendedValue(get_extended_value)
    }
}

impl From<get_policy::Command> for Command {
    fn from(get_policy: get_policy::Command) -> Self {
        Self::GetPolicy(get_policy)
    }
}

impl From<get_value::Command> for Command {
    fn from(get_value: get_value::Command) -> Self {
        Self::GetValue(get_value)
    }
}

impl From<read_attribute::Command> for Command {
    fn from(read_attribute: read_attribute::Command) -> Self {
        Self::ReadAttribute(read_attribute)
    }
}

impl From<send_pan_id_update::Command> for Command {
    fn from(send_pan_id_update: send_pan_id_update::Command) -> Self {
        Self::SendPanIdUpdate(send_pan_id_update)
    }
}

impl From<set_configuration_value::Command> for Command {
    fn from(set_configuration_value: set_configuration_value::Command) -> Self {
        Self::SetConfigurationValue(set_configuration_value)
    }
}

impl From<set_passive_ack_config::Command> for Command {
    fn from(set_passive_ack_config: set_passive_ack_config::Command) -> Self {
        Self::SetPassiveAckConfig(set_passive_ack_config)
    }
}

impl From<set_policy::Command> for Command {
    fn from(set_policy: set_policy::Command) -> Self {
        Self::SetPolicy(set_policy)
    }
}

impl From<set_value::Command> for Command {
    fn from(set_value: set_value::Command) -> Self {
        Self::SetValue(set_value)
    }
}

impl From<write_attribute::Command> for Command {
    fn from(write_attribute: write_attribute::Command) -> Self {
        Self::WriteAttribute(write_attribute)
    }
}

impl Writable for Command {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::AddEndpoint(add_endpoint) => add_endpoint.write_to(dst),
            Self::GetConfigurationValue(get_configuration_value) => {
                get_configuration_value.write_to(dst)
            }
            Self::GetExtendedValue(get_extended_value) => get_extended_value.write_to(dst),
            Self::GetPolicy(get_policy) => get_policy.write_to(dst),
            Self::GetValue(get_value) => get_value.write_to(dst),
            Self::ReadAttribute(read_attribute) => read_attribute.write_to(dst),
            Self::SendPanIdUpdate(send_pan_id_update) => send_pan_id_update.write_to(dst),
            Self::SetConfigurationValue(set_configuration_value) => {
                set_configuration_value.write_to(dst)
            }
            Self::SetPassiveAckConfig(set_passive_ack_config) => {
                set_passive_ack_config.write_to(dst)
            }
            Self::SetPolicy(set_policy) => set_policy.write_to(dst),
            Self::SetValue(set_value) => set_value.write_to(dst),
            Self::WriteAttribute(write_attribute) => write_attribute.write_to(dst),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
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
    WriteAttribute(write_attribute::Response),
}

impl Response {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::AddEndpoint(_) => add_endpoint::ID,
            Self::GetConfigurationValue(_) => get_configuration_value::ID,
            Self::GetExtendedValue(_) => get_extended_value::ID,
            Self::GetPolicy(_) => get_policy::ID,
            Self::GetValue(_) => get_value::ID,
            Self::ReadAttribute(_) => read_attribute::ID,
            Self::SendPanIdUpdate(_) => send_pan_id_update::ID,
            Self::SetConfigurationValue(_) => set_configuration_value::ID,
            Self::SetPassiveAckConfig(_) => set_passive_ack_config::ID,
            Self::SetPolicy(_) => set_policy::ID,
            Self::SetValue(_) => set_value::ID,
            Self::WriteAttribute(_) => write_attribute::ID,
        }
    }
}

impl From<add_endpoint::Response> for Response {
    fn from(add_endpoint: add_endpoint::Response) -> Self {
        Self::AddEndpoint(add_endpoint)
    }
}

impl From<get_configuration_value::Response> for Response {
    fn from(get_configuration_value: get_configuration_value::Response) -> Self {
        Self::GetConfigurationValue(get_configuration_value)
    }
}

impl From<get_extended_value::Response> for Response {
    fn from(get_extended_value: get_extended_value::Response) -> Self {
        Self::GetExtendedValue(get_extended_value)
    }
}

impl From<get_policy::Response> for Response {
    fn from(get_policy: get_policy::Response) -> Self {
        Self::GetPolicy(get_policy)
    }
}

impl From<get_value::Response> for Response {
    fn from(get_value: get_value::Response) -> Self {
        Self::GetValue(get_value)
    }
}

impl From<read_attribute::Response> for Response {
    fn from(read_attribute: read_attribute::Response) -> Self {
        Self::ReadAttribute(read_attribute)
    }
}

impl From<send_pan_id_update::Response> for Response {
    fn from(send_pan_id_update: send_pan_id_update::Response) -> Self {
        Self::SendPanIdUpdate(send_pan_id_update)
    }
}

impl From<set_configuration_value::Response> for Response {
    fn from(set_configuration_value: set_configuration_value::Response) -> Self {
        Self::SetConfigurationValue(set_configuration_value)
    }
}

impl From<set_passive_ack_config::Response> for Response {
    fn from(set_passive_ack_config: set_passive_ack_config::Response) -> Self {
        Self::SetPassiveAckConfig(set_passive_ack_config)
    }
}

impl From<set_policy::Response> for Response {
    fn from(set_policy: set_policy::Response) -> Self {
        Self::SetPolicy(set_policy)
    }
}

impl From<set_value::Response> for Response {
    fn from(set_value: set_value::Response) -> Self {
        Self::SetValue(set_value)
    }
}

impl From<write_attribute::Response> for Response {
    fn from(write_attribute: write_attribute::Response) -> Self {
        Self::WriteAttribute(write_attribute)
    }
}

impl Writable for Response {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::AddEndpoint(add_endpoint) => add_endpoint.write_to(dst),
            Self::GetConfigurationValue(get_configuration_value) => {
                get_configuration_value.write_to(dst)
            }
            Self::GetExtendedValue(get_extended_value) => get_extended_value.write_to(dst),
            Self::GetPolicy(get_policy) => get_policy.write_to(dst),
            Self::GetValue(get_value) => get_value.write_to(dst),
            Self::ReadAttribute(read_attribute) => read_attribute.write_to(dst),
            Self::SendPanIdUpdate(send_pan_id_update) => send_pan_id_update.write_to(dst),
            Self::SetConfigurationValue(set_configuration_value) => {
                set_configuration_value.write_to(dst)
            }
            Self::SetPassiveAckConfig(set_passive_ack_config) => {
                set_passive_ack_config.write_to(dst)
            }
            Self::SetPolicy(set_policy) => set_policy.write_to(dst),
            Self::SetValue(set_value) => set_value.write_to(dst),
            Self::WriteAttribute(write_attribute) => write_attribute.write_to(dst),
        }
    }
}
