use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::frame::Parameter;

const ID: u16 = 0x00B1;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
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

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
