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
pub enum Command {
    End(end::Command),
    GetChannel(get_channel::Command),
    GetPower(get_power::Command),
    SendPacket(send_packet::Command),
    SetChannel(set_channel::Command),
    SetPower(set_power::Command),
    Start(start::Command),
    StartStream(start_stream::Command),
    StartTone(start_tone::Command),
    StopStream(stop_stream::Command),
    StopTone(stop_tone::Command),
}

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
