use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;
use crate::Resolve;

const ID: u16 = 0x00BA;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    logical_channel: u8,
}

impl Response {
    #[must_use]
    pub const fn logical_channel(&self) -> u8 {
        self.logical_channel
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
