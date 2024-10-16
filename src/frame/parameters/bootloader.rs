//! Bootloader Frames

pub(crate) mod aes_encrypt;
pub mod get_standalone_bootloader_version_plat_micro_phy;
pub mod handler;
pub(crate) mod launch_standalone_bootloader;
pub(crate) mod override_current_channel;
pub(crate) mod send_bootload_message;
