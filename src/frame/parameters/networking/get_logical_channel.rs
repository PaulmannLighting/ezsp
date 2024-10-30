//! Parameters for the [`Networking::get_logical_channel`](crate::Networking::get_logical_channel) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x00BA;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    logical_channel: u8,
}

impl Response {
    /// Returns the logical channel.
    #[must_use]
    pub const fn logical_channel(&self) -> u8 {
        self.logical_channel
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
