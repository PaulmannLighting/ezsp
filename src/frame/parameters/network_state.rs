use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberNetworkStatus};

pub const ID: u16 = 0x0018;

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
    status: EmberNetworkStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberNetworkStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberNetworkStatus {
        self.status
    }
}
