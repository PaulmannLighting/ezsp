use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberCounterType};

pub const ID: u16 = 0x00F2;

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
    typ: EmberCounterType,
}

impl Response {
    #[must_use]
    pub const fn new(typ: EmberCounterType) -> Self {
        Self { typ }
    }

    #[must_use]
    pub const fn typ(&self) -> EmberCounterType {
        self.typ
    }
}
