use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_traits::FromPrimitive;

pub const ID: u16 = 0x0078;

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
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    #[must_use]
    pub const fn status(&self) -> Option<Status> {
        Status::from_u8(self.status)
    }
}
