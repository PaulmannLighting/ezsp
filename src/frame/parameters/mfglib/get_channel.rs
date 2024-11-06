//! Parameters for the [`Mfglib::get_channel`](crate::Mfglib::get_channel) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x008B;

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
    /// Returns the channel.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
