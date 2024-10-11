use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x008D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    power: i8,
}

impl Response {
    #[must_use]
    pub const fn power(&self) -> i8 {
        self.power
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
