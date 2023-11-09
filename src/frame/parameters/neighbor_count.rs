use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{};

pub const ID: u16 = 0x007A;

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
    value: u8,
}

impl Response {
    #[must_use]
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    #[must_use]
    pub const fn value(&self) -> u8 {
        self.value
    }
}
