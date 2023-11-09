use crate::types::{EmberKeyData, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0073;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    key: EmberKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(key: EmberKeyData) -> Self {
        Self { key }
    }

    #[must_use]
    pub const fn key(&self) -> EmberKeyData {
        self.key
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
