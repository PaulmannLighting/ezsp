//! Mfglib Frames

pub(crate) mod end;
pub(crate) mod get_channel;
pub(crate) mod get_power;
pub mod handler;
pub(crate) mod send_packet;
pub(crate) mod set_channel;
pub(crate) mod set_power;
pub(crate) mod start;
pub(crate) mod start_stream;
pub(crate) mod start_tone;
pub(crate) mod stop_stream;
pub(crate) mod stop_tone;

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
