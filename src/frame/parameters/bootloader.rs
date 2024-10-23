//! Bootloader Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

pub mod aes_encrypt;
pub mod get_standalone_bootloader_version_plat_micro_phy;
pub mod handler;
pub mod launch_standalone_bootloader;
pub mod override_current_channel;
pub mod send_bootload_message;

/// `EZSP` response parameters for bootloader frames.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Response {
    /// Response parameters for [`Bootloader::aes_encrypt()`](crate::Bootloader::aes_encrypt).
    AesEncrypt(aes_encrypt::Response),
    /// Response parameters for [`Bootloader::get_standalone_bootloader_version_plat_micro_phy()`](crate::Bootloader::get_standalone_bootloader_version_plat_micro_phy).
    GetStandaloneBootloaderVersionPlatMicroPhy(
        get_standalone_bootloader_version_plat_micro_phy::Response,
    ),
    /// Response parameters for [`Bootloader::launch_standalone_bootloader()`](crate::Bootloader::launch_standalone_bootloader).
    LaunchStandaloneBootloader(launch_standalone_bootloader::Response),
    /// Response parameters for [`Bootloader::override_current_channel()`](crate::Bootloader::override_current_channel).
    OverrideCurrentChannel(override_current_channel::Response),
    /// Response parameters for [`Bootloader::send_bootload_message()`](crate::Bootloader::send_bootload_message).
    SendBootloadMessage(send_bootload_message::Response),
    /// Callback handlers.
    Handler(handler::Handler),
}

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <aes_encrypt::Response as Parameter>::ID => {
                Ok(Self::AesEncrypt(aes_encrypt::Response::from_le_stream_exact(stream)?))
            }
            <get_standalone_bootloader_version_plat_micro_phy::Response as Parameter>::ID => Ok(Self::GetStandaloneBootloaderVersionPlatMicroPhy(
                get_standalone_bootloader_version_plat_micro_phy::Response::from_le_stream_exact(stream)?,
            )),
            <launch_standalone_bootloader::Response as Parameter>::ID => Ok(Self::LaunchStandaloneBootloader(
                launch_standalone_bootloader::Response::from_le_stream_exact(stream)?,
            )),
            <override_current_channel::Response as Parameter>::ID => Ok(Self::OverrideCurrentChannel(
                override_current_channel::Response::from_le_stream_exact(stream)?,
            )),
            <send_bootload_message::Response as Parameter>::ID => Ok(Self::SendBootloadMessage(
                send_bootload_message::Response::from_le_stream_exact(stream)?,
            )),
            <handler::BootloadTransmitComplete as Parameter>::ID => Ok(Self::Handler(
                handler::Handler::BootloadTransmitComplete(
                    handler::BootloadTransmitComplete::from_le_stream_exact(stream)?,
                ),
            )),
            <handler::IncomingBootloadMessage as Parameter>::ID => Ok(Self::Handler(
                handler::Handler::IncomingBootloadMessage(
                    handler::IncomingBootloadMessage::from_le_stream_exact(stream)?,
                ),
            )),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
