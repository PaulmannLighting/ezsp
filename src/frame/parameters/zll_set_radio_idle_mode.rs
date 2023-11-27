use crate::ember::radio::PowerMode;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00D4;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    mode: u8,
}

impl Command {
    #[must_use]
    pub fn new(mode: PowerMode) -> Self {
        Self { mode: mode.into() }
    }

    pub fn mode(&self) -> Result<PowerMode, u8> {
        PowerMode::try_from(self.mode)
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
