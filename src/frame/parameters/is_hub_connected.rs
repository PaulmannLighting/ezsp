use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool};

pub const ID: u16 = 0x00E6;

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
    is_hub_connected: bool,
}

impl Response {
    #[must_use]
    pub const fn new(is_hub_connected: bool) -> Self {
        Self { is_hub_connected }
    }

    #[must_use]
    pub const fn is_hub_connected(&self) -> bool {
        self.is_hub_connected
    }
}
