//! Parameters for the [`Networking::get_radio_channel`](crate::Networking::get_radio_channel) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00FF;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    channel: u8,
}

impl Response {
    /// Returns the radio channel.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
