use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00FF;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    channel: u8,
}

impl Response {
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
