use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x008D;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    power: i8,
}

impl Response {
    #[must_use]
    pub const fn new(power: i8) -> Self {
        Self { power }
    }

    #[must_use]
    pub const fn power(&self) -> i8 {
        self.power
    }
}
