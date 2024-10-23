//! Configuration Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

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

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        const VERSION_ID: u16 = <version::Response as Parameter>::ID as u16;

        match id {
            <add_endpoint::Response as Parameter>::ID => Ok(Self::AddEndpoint(
                add_endpoint::Response::from_le_stream_exact(stream)?,
            )),
            <get_configuration_value::Response as Parameter>::ID => {
                Ok(Self::GetConfigurationValue(
                    get_configuration_value::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_extended_value::Response as Parameter>::ID => Ok(Self::GetExtendedValue(
                get_extended_value::Response::from_le_stream_exact(stream)?,
            )),
            <get_policy::Response as Parameter>::ID => Ok(Self::GetPolicy(
                get_policy::Response::from_le_stream_exact(stream)?,
            )),
            <get_value::Response as Parameter>::ID => Ok(Self::GetValue(
                get_value::Response::from_le_stream_exact(stream)?,
            )),
            <read_attribute::Response as Parameter>::ID => Ok(Self::ReadAttribute(
                read_attribute::Response::from_le_stream_exact(stream)?,
            )),
            <send_pan_id_update::Response as Parameter>::ID => Ok(Self::SendPanIdUpdate(
                send_pan_id_update::Response::from_le_stream_exact(stream)?,
            )),
            <set_configuration_value::Response as Parameter>::ID => {
                Ok(Self::SetConfigurationValue(
                    set_configuration_value::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_passive_ack_config::Response as Parameter>::ID => Ok(Self::SetPassiveAckConfig(
                set_passive_ack_config::Response::from_le_stream_exact(stream)?,
            )),
            <set_policy::Response as Parameter>::ID => Ok(Self::SetPolicy(
                set_policy::Response::from_le_stream_exact(stream)?,
            )),
            <set_value::Response as Parameter>::ID => Ok(Self::SetValue(
                set_value::Response::from_le_stream_exact(stream)?,
            )),
            VERSION_ID => Ok(Self::Version(version::Response::from_le_stream_exact(
                stream,
            )?)),
            <write_attribute::Response as Parameter>::ID => Ok(Self::WriteAttribute(
                write_attribute::Response::from_le_stream_exact(stream)?,
            )),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
