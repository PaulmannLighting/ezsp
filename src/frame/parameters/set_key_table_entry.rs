use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool,EmberKeyData,EmberEUI64,EmberStatus};

pub const ID: u16 = 0x0072;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    index: u8,
    address: EmberEUI64,
    link_key: bool,
    key_data: EmberKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, address: EmberEUI64, link_key: bool, key_data: EmberKeyData) -> Self {
        Self { index, address, link_key, key_data }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }


    #[must_use]
    pub const fn address(&self) -> EmberEUI64 {
        self.address
    }


    #[must_use]
    pub const fn link_key(&self) -> bool {
        self.link_key
    }


    #[must_use]
    pub const fn key_data(&self) -> EmberKeyData {
        self.key_data
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
