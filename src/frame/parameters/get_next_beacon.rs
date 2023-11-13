use crate::ember::beacon::Data;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0004;

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
    beacon: Data,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, beacon: Data) -> Self {
        Self {
            status: status.into(),
            beacon,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn beacon(&self) -> &Data {
        &self.beacon
    }
}
