use crate::ezsp::config::Id;
use crate::ezsp::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0052;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    config_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(config_id: Id) -> Self {
        Self {
            config_id: config_id.into(),
        }
    }

    pub fn config_id(&self) -> Result<Id, u8> {
        Id::try_from(self.config_id)
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    value: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, value: u16) -> Self {
        Self {
            status: status.into(),
            value,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }
}
