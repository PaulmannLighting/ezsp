//! Bootloader Frames

pub use self::aes_encrypt::Response as AesEncrypt;
pub use self::get_standalone_bootloader_version_plat_micro_phy::Response as GetStandaloneBootloaderVersionPlatMicroPhy;
pub use self::launch_standalone_bootloader::Response as LaunchStandaloneBootloader;
pub use self::send_bootload_message::Response as SendBootloadMessage;

pub mod aes_encrypt;
pub mod get_standalone_bootloader_version_plat_micro_phy;
pub mod handler;
#[expect(clippy::module_name_repetitions)]
pub mod launch_standalone_bootloader;
pub mod send_bootload_message;

crate::frame::parameters::command_enum!(
    Command,
    AesEncrypt(aes_encrypt::Command),
    GetStandaloneBootloaderVersionPlatMicroPhy(
        get_standalone_bootloader_version_plat_micro_phy::Command
    ),
    LaunchStandaloneBootloader(launch_standalone_bootloader::Command),
    SendBootloadMessage(send_bootload_message::Command),
);

crate::frame::parameters::parameter_enum!(
    Response,
    AesEncrypt,
    GetStandaloneBootloaderVersionPlatMicroPhy,
    LaunchStandaloneBootloader,
    SendBootloadMessage
);
