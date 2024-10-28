//! Bootloader Frames

pub mod aes_encrypt;
pub mod get_standalone_bootloader_version_plat_micro_phy;
pub mod handler;
#[allow(clippy::module_name_repetitions)]
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
}
