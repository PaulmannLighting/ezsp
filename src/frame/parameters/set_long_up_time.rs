use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool};

pub const ID: u16 = 0x00E3;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    has_long_up_time: bool,
}

impl Command {
    #[must_use]
    pub const fn new(has_long_up_time: bool) -> Self {
        Self { has_long_up_time }
    }

    #[must_use]
    pub const fn has_long_up_time(&self) -> bool {
        self.has_long_up_time
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
