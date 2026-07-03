//! `Mfglib` frames.

pub use self::end::Response as End;
pub use self::get_channel::Response as GetChannel;
pub use self::get_power::Response as GetPower;
pub use self::send_packet::Response as SendPacket;
pub use self::set_channel::Response as SetChannel;
pub use self::set_power::Response as SetPower;
pub use self::start::Response as Start;
pub use self::start_stream::Response as StartStream;
pub use self::start_tone::Response as StartTone;
pub use self::stop_stream::Response as StopStream;
pub use self::stop_tone::Response as StopTone;

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

crate::frame::parameters::parameter_enum!(
    Response,
    End,
    GetChannel,
    GetPower,
    SendPacket,
    SetChannel,
    SetPower,
    Start,
    StartStream,
    StartTone,
    StopStream,
    StopTone
);
