use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x008F;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    mode: u8,
}

impl Command {
    #[must_use]
    pub const fn new(mode: u8) -> Self {
        Self { mode }
    }

    #[must_use]
    pub const fn mode(&self) -> u8 {
        self.mode
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
