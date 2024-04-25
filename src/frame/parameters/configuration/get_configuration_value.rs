use crate::error::Resolve;
use crate::ezsp::config::Id;
use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::Error;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0052;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    config_id: u8,
}

impl Command {
    #[must_use]
    pub fn new(config_id: Id) -> Self {
        Self {
            config_id: config_id.into(),
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
    value: u16,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = u16;

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve().map(|()| self.value)
    }
}
