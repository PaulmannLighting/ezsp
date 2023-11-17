use crate::ember::duty_cycle::Limits;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0040;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    limits: Limits,
}

impl Command {
    #[must_use]
    pub const fn new(limits: Limits) -> Self {
        Self { limits }
    }

    #[must_use]
    pub const fn limits(&self) -> &Limits {
        &self.limits
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
