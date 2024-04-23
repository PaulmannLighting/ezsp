use crate::ember::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x008F;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    mode: u8,
}

impl Command {
    #[must_use]
    pub const fn new(mode: u8) -> Self {
        Self { mode }
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
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
