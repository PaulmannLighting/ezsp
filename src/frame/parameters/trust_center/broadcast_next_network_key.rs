use crate::ember::key::Data;
use crate::ember::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0073;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    key: Data,
}

impl Command {
    #[must_use]
    pub const fn new(key: Data) -> Self {
        Self { key }
    }
}

impl Parameter<u16> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}
