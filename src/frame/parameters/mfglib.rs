//! `Mfglib` frames.

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

/// Response parameters for the `Mfglib` commands.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response to the `end` command.
    End(end::Response),
    /// Response to the `get_channel` command.
    GetChannel(get_channel::Response),
    /// Response to the `get_power` command.
    GetPower(get_power::Response),
    /// Response to the `send_packet` command.
    SendPacket(send_packet::Response),
    /// Response to the `set_channel` command.
    SetChannel(set_channel::Response),
    /// Response to the `set_power` command.
    SetPower(set_power::Response),
    /// Response to the `start` command.
    Start(start::Response),
    /// Response to the `start_stream` command.
    StartStream(start_stream::Response),
    /// Response to the `start_tone` command.
    StartTone(start_tone::Response),
    /// Response to the `stop_stream` command.
    StopStream(stop_stream::Response),
    /// Response to the `stop_tone` command.
    StopTone(stop_tone::Response),
}
