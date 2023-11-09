use crate::types::EmberStatus;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0049;

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
    status: EmberStatus,
    value: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, value: u16) -> Self {
        Self { status, value }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }
}
