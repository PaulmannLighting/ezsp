use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0105;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    config: u8,
    min_acks_needed: u8,
}

impl Command {
    #[must_use]
    pub const fn new(config: u8, min_acks_needed: u8) -> Self {
        Self {
            config,
            min_acks_needed,
        }
    }

    #[must_use]
    pub const fn config(&self) -> u8 {
        self.config
    }

    #[must_use]
    pub const fn min_acks_needed(&self) -> u8 {
        self.min_acks_needed
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
