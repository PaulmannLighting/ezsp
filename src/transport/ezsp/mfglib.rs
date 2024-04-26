use std::future::Future;

use crate::error::Resolve;
use crate::frame::parameters::mfglib::{end, get_channel, get_power, send_packet};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};

pub trait Mfglib {
    /// Deactivate use of `Mfglib` test routines; restores the hardware to the state it was in prior
    /// to [`start()`](Self::start) and stops receiving packets started by [`start()`](Self::start)
    /// at the same time.
    fn end(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Returns the current radio channel, as previously set via [`set_channel()`](Self::set_channel).
    fn get_channel(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Returns the current radio power setting, as previously set via [`set_power()`](Self::set_power).
    fn get_power(&self) -> impl Future<Output = Result<i8, Error>> + Send;

    /// Sends a single packet consisting of the following bytes:
    ///     * `packetLength`
    ///     * `packetContents[0]`
    ///     * ...
    ///     * `packetContents[packetLength - 3]`
    ///     * `CRC[0]`
    ///     * `CRC[1]`
    ///
    /// The total number of bytes sent is packetLength + 1.
    /// The radio replaces the last two bytes of `content` with the 16-bit CRC for the packet.
    fn send_packet(
        &self,
        content: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Mfglib for T
where
    T: Transport,
{
    async fn end(&self) -> Result<(), Error> {
        self.communicate::<_, end::Response>(end::Command)
            .await?
            .resolve()
    }

    async fn get_channel(&self) -> Result<u8, Error> {
        self.communicate::<_, get_channel::Response>(get_channel::Command)
            .await
            .map(|response| response.channel())
    }

    async fn get_power(&self) -> Result<i8, Error> {
        self.communicate::<_, get_power::Response>(get_power::Command)
            .await
            .map(|response| response.power())
    }

    async fn send_packet(&self, content: ByteSizedVec<u8>) -> Result<(), Error> {
        self.communicate::<_, send_packet::Response>(send_packet::Command::new(content))
            .await?
            .resolve()
    }
}
