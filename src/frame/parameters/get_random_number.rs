use crate::ember::Status;
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
    status: u8,
    value: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, value: u16) -> Self {
        Self {
            status: status.into(),
            value,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }
}
