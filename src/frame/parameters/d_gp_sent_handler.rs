use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00C7;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    gpep_handle: u8,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn gpep_handle(&self) -> u8 {
        self.gpep_handle
    }
}
