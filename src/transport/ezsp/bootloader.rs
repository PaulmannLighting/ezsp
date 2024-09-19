use std::future::Future;

use crate::ember::Eui64;
use crate::frame::parameters::bootloader::{
    aes_encrypt, get_standalone_bootloader_version_plat_micro_phy, launch_standalone_bootloader,
    override_current_channel, send_bootload_message,
};
use crate::types::ByteSizedVec;
use crate::Resolve;
use crate::{Error, Transport};

pub trait Bootloader {
    /// Perform AES encryption on `plaintext` using `key`.
    fn aes_encrypt(
        &self,
        plaintext: [u8; 16],
        key: [u8; 16],
    ) -> impl Future<Output = Result<[u8; 16], Error>> + Send;

    /// Detects if the standalone bootloader is installed, and if so returns the installed version.
    ///
    /// If not return `0xffff`. A returned version of `0x1234` would indicate version 1.2 build 34.
    /// Also return the node's version of `PLAT`, `MICRO` and `PHY`.
    fn get_standalone_bootloader_version_plat_micro_phy(
        &self,
    ) -> impl Future<
        Output = Result<get_standalone_bootloader_version_plat_micro_phy::Response, Error>,
    > + Send;

    /// Quits the current application and launches the standalone bootloader (if installed).
    ///
    /// The function returns an error if the standalone bootloader is not present.
    fn launch_standalone_bootloader(
        &self,
        mode: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// A bootloader method for selecting the radio channel.
    ///
    /// This routine only works for sending and receiving bootload packets.
    /// Does not correctly do ZigBee stack changes.
    ///
    /// NOTE: this API is not safe to call on multi-network devices and it will return failure when so.
    /// Use of the ember/ezspSetRadioChannel APIs are multi-network safe and are recommended instead.
    fn override_current_channel(
        &self,
        channel: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Transmits the given bootload message to a neighboring node using a specific 802.15.4 header
    /// that allows the EmberZNet stack as well as the bootloader to recognize the message,
    /// but will not interfere with other ZigBee stacks.
    fn send_bootload_message(
        &self,
        broadcast: bool,
        dest_eui64: Eui64,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Bootloader for T
where
    T: Transport,
{
    async fn aes_encrypt(&self, plaintext: [u8; 16], key: [u8; 16]) -> Result<[u8; 16], Error> {
        self.communicate::<_, aes_encrypt::Response>(aes_encrypt::Command::new(plaintext, key))
            .await
            .map(|response| response.ciphertext())
    }

    async fn get_standalone_bootloader_version_plat_micro_phy(
        &self,
    ) -> Result<get_standalone_bootloader_version_plat_micro_phy::Response, Error> {
        self.communicate::<_, get_standalone_bootloader_version_plat_micro_phy::Response>(
            get_standalone_bootloader_version_plat_micro_phy::Command,
        )
        .await
    }

    async fn launch_standalone_bootloader(&self, mode: u8) -> Result<(), Error> {
        self.communicate::<_, launch_standalone_bootloader::Response>(
            launch_standalone_bootloader::Command::new(mode),
        )
        .await?
        .resolve()
    }

    async fn override_current_channel(&self, channel: u8) -> Result<(), Error> {
        self.communicate::<_, override_current_channel::Response>(
            override_current_channel::Command::new(channel),
        )
        .await?
        .resolve()
    }

    async fn send_bootload_message(
        &self,
        broadcast: bool,
        dest_eui64: Eui64,
        message: ByteSizedVec<u8>,
    ) -> Result<(), Error> {
        self.communicate::<_, send_bootload_message::Response>(send_bootload_message::Command::new(
            broadcast, dest_eui64, message,
        ))
        .await?
        .resolve()
    }
}
