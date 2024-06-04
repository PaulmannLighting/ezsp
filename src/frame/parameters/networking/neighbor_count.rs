use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::frame::Parameter;

const ID: u16 = 0x007A;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    value: u8,
}

impl Response {
    #[must_use]
    pub const fn value(&self) -> u8 {
        self.value
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
