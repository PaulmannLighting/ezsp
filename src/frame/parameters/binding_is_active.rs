use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x002E;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    active: bool,
}

impl Response {
    #[must_use]
    pub const fn new(active: bool) -> Self {
        Self { active }
    }

    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}
