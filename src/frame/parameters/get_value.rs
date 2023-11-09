use crate::types::{EzspStatus, EzspValueId};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00AA;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    value_id: EzspValueId,
}

impl Command {
    #[must_use]
    pub const fn new(value_id: EzspValueId) -> Self {
        Self { value_id }
    }

    #[must_use]
    pub const fn value_id(&self) -> EzspValueId {
        self.value_id
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EzspStatus,
    value_length: u8,
    value: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus, value_length: u8, value: ByteSizedVec<u8>) -> Self {
        Self {
            status,
            value_length,
            value,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }

    #[must_use]
    pub const fn value_length(&self) -> u8 {
        self.value_length
    }

    #[must_use]
    pub const fn value(&self) -> ByteSizedVec<u8> {
        self.value
    }
}
