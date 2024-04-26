use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::frame::Parameter;

const ID: u16 = 0x008B;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    channel: u8,
}

impl Response {
    pub fn channel(&self) -> u8 {
        self.channel
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
