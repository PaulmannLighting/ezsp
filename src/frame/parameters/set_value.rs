use crate::ezsp::value::Id;
use crate::ezsp::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00AB;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    value_id: u8,
    value: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub fn new(value_id: Id, value: ByteSizedVec<u8>) -> Self {
        Self {
            value_id: value_id.into(),
            value,
        }
    }

    pub fn value_id(&self) -> Result<Id, u8> {
        Id::try_from(self.value_id)
    }

    #[must_use]
    pub const fn value(&self) -> &ByteSizedVec<u8> {
        &self.value
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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
