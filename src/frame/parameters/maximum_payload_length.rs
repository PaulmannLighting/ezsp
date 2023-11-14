use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0033;

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
    aps_length: u8,
}

impl Response {
    #[must_use]
    pub const fn new(aps_length: u8) -> Self {
        Self { aps_length }
    }

    #[must_use]
    pub const fn aps_length(&self) -> u8 {
        self.aps_length
    }
}
