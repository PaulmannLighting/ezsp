use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x006C;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    max_attempts: u8,
}

impl Command {
    #[must_use]
    pub const fn new(max_attempts: u8) -> Self {
        Self { max_attempts }
    }

    #[must_use]
    pub const fn max_attempts(&self) -> u8 {
        self.max_attempts
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
