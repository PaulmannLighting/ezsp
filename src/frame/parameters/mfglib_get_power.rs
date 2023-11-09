use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{int8_t};

pub const ID: u16 = 0x008D;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    power: int8_t,
}

impl Response {
    #[must_use]
    pub const fn new(power: int8_t) -> Self {
        Self { power }
    }

    #[must_use]
    pub const fn power(&self) -> int8_t {
        self.power
    }
}
