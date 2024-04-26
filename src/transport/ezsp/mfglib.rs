use std::future::Future;

use crate::error::Resolve;
use crate::frame::parameters::mfglib::{
    end, get_channel, get_power, send_packet, set_channel, set_power, start, start_stream,
    start_tone, stop_stream, stop_tone,
};
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

    /// Sets the radio channel.
    ///
    /// Calibration occurs if this is the first time the channel has been used.
    fn set_channel(&self, channel: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// First select the transmit power mode, and then include a method for selecting the radio transmit power.
    ///
    /// The valid power settings depend upon the specific radio in use.
    /// Ember radios have discrete power settings, and then requested power is rounded to a valid
    /// power setting; the actual power output is available to the caller via [`get_power()`](Self::get_power).
    fn set_power(
        &self,
        tx_power_mode: u16,
        power: i8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Activate use of mfglib test routines and enables the radio receiver to report packets it
    /// receives to the [`rx_handler`] callback.
    ///
    /// These packets will not be passed up with a CRC failure.
    /// All other mfglib functions will return an error until the [`start()`](Self::start) has been called.
    fn start(&self, rx_callback: bool) -> impl Future<Output = Result<(), Error>> + Send;

    /// Starts transmitting a random stream of characters.
    ///
    /// This is so that the radio modulation can be measured.
    fn start_stream(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Starts transmitting an unmodulated tone on the currently set channel and power level.
    ///
    /// Upon successful return, the tone will be transmitting.
    /// To stop transmitting tone, application must call [`stop_tone()`](Self::stop_tone), allowing
    /// it the flexibility to determine its own criteria or tone duration (time, event, etc.).
    fn start_tone(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Stops transmitting a random stream of characters started by [`start_stream()`](Self::start_stream).
    fn stop_stream(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Stops transmitting tone started by [`start_tone()`](Self::start_tone).
    fn stop_tone(&self) -> impl Future<Output = Result<(), Error>> + Send;
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

    async fn set_channel(&self, channel: u8) -> Result<(), Error> {
        self.communicate::<_, set_channel::Response>(set_channel::Command::new(channel))
            .await?
            .resolve()
    }

    async fn set_power(&self, tx_power_mode: u16, power: i8) -> Result<(), Error> {
        self.communicate::<_, set_power::Response>(set_power::Command::new(tx_power_mode, power))
            .await?
            .resolve()
    }

    async fn start(&self, rx_callback: bool) -> Result<(), Error> {
        self.communicate::<_, start::Response>(start::Command::new(rx_callback))
            .await?
            .resolve()
    }

    async fn start_stream(&self) -> Result<(), Error> {
        self.communicate::<_, start_stream::Response>(start_stream::Command)
            .await?
            .resolve()
    }

    async fn start_tone(&self) -> Result<(), Error> {
        self.communicate::<_, start_tone::Response>(start_tone::Command)
            .await?
            .resolve()
    }

    async fn stop_stream(&self) -> Result<(), Error> {
        self.communicate::<_, stop_stream::Response>(stop_stream::Command)
            .await?
            .resolve()
    }

    async fn stop_tone(&self) -> Result<(), Error> {
        self.communicate::<_, stop_tone::Response>(stop_tone::Command)
            .await?
            .resolve()
    }
}
