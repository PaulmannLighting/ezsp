use crate::ezsp::config::Id;
use crate::ezsp::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0053;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    config_id: u8,
    value: u16,
}

impl Command {
    #[must_use]
    pub fn new(config_id: Id, value: u16) -> Self {
        Self {
            config_id: config_id.into(),
            value,
        }
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
