//! Mfglib Frames

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

#[allow(variant_size_differences)]
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
}
