use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x00FF;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Identified for Command {
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

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
