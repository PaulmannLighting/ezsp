use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool};

pub const ID: u16 = 0x00E7;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    enabled: bool,
}

impl Command {
    #[must_use]
    pub const fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}
