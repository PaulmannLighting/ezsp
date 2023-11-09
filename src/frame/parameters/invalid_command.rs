use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EzspStatus};

pub const ID: u16 = 0x0058;

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
    reason: EzspStatus,
}

impl Response {
    #[must_use]
    pub const fn new(reason: EzspStatus) -> Self {
        Self { reason }
    }

    #[must_use]
    pub const fn reason(&self) -> EzspStatus {
        self.reason
    }
}
