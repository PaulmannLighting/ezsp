use crate::ember::beacon::Iterator;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x003D;

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
    beacon_iterator: Iterator,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, beacon_iterator: Iterator) -> Self {
        Self {
            status: status.into(),
            beacon_iterator,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn beacon_iterator(&self) -> &Iterator {
        &self.beacon_iterator
    }
}
