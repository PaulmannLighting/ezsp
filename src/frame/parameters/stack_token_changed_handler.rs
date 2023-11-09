use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{};

pub const ID: u16 = 0x000D;

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
    token_address: u16,
}

impl Response {
    #[must_use]
    pub const fn new(token_address: u16) -> Self {
        Self { token_address }
    }

    #[must_use]
    pub const fn token_address(&self) -> u16 {
        self.token_address
    }
}
