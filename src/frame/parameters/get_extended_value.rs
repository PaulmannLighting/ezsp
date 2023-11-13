use crate::ezsp::value::ExtendedId;
use crate::ezsp::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0003;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    value_id: u8,
    characteristics: u32,
}

impl Command {
    #[must_use]
    pub const fn new(value_id: ExtendedId, characteristics: u32) -> Self {
        Self {
            value_id: value_id.into(),
            characteristics,
        }
    }

    pub fn value_id(&self) -> Result<ExtendedId, u8> {
        ExtendedId::try_from(self.value_id)
    }

    #[must_use]
    pub const fn characteristics(&self) -> u32 {
        self.characteristics
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    value_length: u8,
    value: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, value_length: u8, value: ByteSizedVec<u8>) -> Self {
        Self {
            status: status.into(),
            value_length,
            value,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn value_length(&self) -> u8 {
        self.value_length
    }

    #[must_use]
    pub const fn value(&self) -> &ByteSizedVec<u8> {
        &self.value
    }
}
