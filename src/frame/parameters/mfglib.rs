//! Mfglib Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

pub mod end;
pub mod get_channel;
pub mod get_power;
pub mod handler;
pub mod send_packet;
pub mod set_channel;
pub mod set_power;
pub mod start;
pub mod start_stream;
pub mod start_tone;
pub mod stop_stream;
pub mod stop_tone;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    End(end::Response),
    GetChannel(get_channel::Response),
    GetPower(get_power::Response),
    SendPacket(send_packet::Response),
    SetChannel(set_channel::Response),
    SetPower(set_power::Response),
    Start(start::Response),
    StartStream(start_stream::Response),
    StartTone(start_tone::Response),
    StopStream(stop_stream::Response),
    StopTone(stop_tone::Response),
    Handler(handler::Handler),
}

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <end::Response as Parameter>::ID => {
                Ok(Self::End(end::Response::from_le_stream_exact(stream)?))
            }
            <get_channel::Response as Parameter>::ID => Ok(Self::GetChannel(
                get_channel::Response::from_le_stream_exact(stream)?,
            )),
            <get_power::Response as Parameter>::ID => Ok(Self::GetPower(
                get_power::Response::from_le_stream_exact(stream)?,
            )),
            <send_packet::Response as Parameter>::ID => Ok(Self::SendPacket(
                send_packet::Response::from_le_stream_exact(stream)?,
            )),
            <set_channel::Response as Parameter>::ID => Ok(Self::SetChannel(
                set_channel::Response::from_le_stream_exact(stream)?,
            )),
            <set_power::Response as Parameter>::ID => Ok(Self::SetPower(
                set_power::Response::from_le_stream_exact(stream)?,
            )),
            <start::Response as Parameter>::ID => {
                Ok(Self::Start(start::Response::from_le_stream_exact(stream)?))
            }
            <start_stream::Response as Parameter>::ID => Ok(Self::StartStream(
                start_stream::Response::from_le_stream_exact(stream)?,
            )),
            <start_tone::Response as Parameter>::ID => Ok(Self::StartTone(
                start_tone::Response::from_le_stream_exact(stream)?,
            )),
            <stop_stream::Response as Parameter>::ID => Ok(Self::StopStream(
                stop_stream::Response::from_le_stream_exact(stream)?,
            )),
            <stop_tone::Response as Parameter>::ID => Ok(Self::StopTone(
                stop_tone::Response::from_le_stream_exact(stream)?,
            )),
            <handler::Rx as Parameter>::ID => Ok(Self::Handler(handler::Handler::Rx(
                handler::Rx::from_le_stream_exact(stream)?,
            ))),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
