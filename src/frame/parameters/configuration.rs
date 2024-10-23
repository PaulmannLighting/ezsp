//! Configuration Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

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

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        const VERSION_ID: u16 = <version::Response as Parameter>::ID as u16;

        match id {
            <add_endpoint::Response as Parameter>::ID => Ok(Response::AddEndpoint(
                add_endpoint::Response::from_le_stream_exact(stream)?,
            )),
            <get_configuration_value::Response as Parameter>::ID => {
                Ok(Response::GetConfigurationValue(
                    get_configuration_value::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_extended_value::Response as Parameter>::ID => Ok(Response::GetExtendedValue(
                get_extended_value::Response::from_le_stream_exact(stream)?,
            )),
            <get_policy::Response as Parameter>::ID => Ok(Response::GetPolicy(
                get_policy::Response::from_le_stream_exact(stream)?,
            )),
            <get_value::Response as Parameter>::ID => Ok(Response::GetValue(
                get_value::Response::from_le_stream_exact(stream)?,
            )),
            <read_attribute::Response as Parameter>::ID => Ok(Response::ReadAttribute(
                read_attribute::Response::from_le_stream_exact(stream)?,
            )),
            <send_pan_id_update::Response as Parameter>::ID => Ok(Response::SendPanIdUpdate(
                send_pan_id_update::Response::from_le_stream_exact(stream)?,
            )),
            <set_configuration_value::Response as Parameter>::ID => {
                Ok(Response::SetConfigurationValue(
                    set_configuration_value::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_passive_ack_config::Response as Parameter>::ID => {
                Ok(Response::SetPassiveAckConfig(
                    set_passive_ack_config::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_policy::Response as Parameter>::ID => Ok(Response::SetPolicy(
                set_policy::Response::from_le_stream_exact(stream)?,
            )),
            <set_value::Response as Parameter>::ID => Ok(Response::SetValue(
                set_value::Response::from_le_stream_exact(stream)?,
            )),
            VERSION_ID => Ok(Response::Version(version::Response::from_le_stream_exact(
                stream,
            )?)),
            <write_attribute::Response as Parameter>::ID => Ok(Response::WriteAttribute(
                write_attribute::Response::from_le_stream_exact(stream)?,
            )),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
