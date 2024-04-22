use crate::ember::key::Data;
use crate::ember::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0073;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    key: Data,
}

impl Command {
    #[must_use]
    pub const fn new(key: Data) -> Self {
        Self { key }
    }

    #[must_use]
    pub const fn key(&self) -> &Data {
        &self.key
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
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
    const ID: u16 = ID;
}
