use crate::ezsp::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0058;

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
    reason: u8,
}

impl Response {
    #[must_use]
    pub fn new(reason: Status) -> Self {
        Self {
            reason: reason.into(),
        }
    }

    pub fn reason(&self) -> Result<Status, u8> {
        Status::try_from(self.reason)
    }
}
