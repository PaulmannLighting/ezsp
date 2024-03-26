use crate::ember::key::Data;
use crate::ember::{Eui64, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0072;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    index: u8,
    address: Eui64,
    link_key: bool,
    key_data: Data,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, address: Eui64, link_key: bool, key_data: Data) -> Self {
        Self {
            index,
            address,
            link_key,
            key_data,
        }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn address(&self) -> Eui64 {
        self.address
    }

    #[must_use]
    pub const fn link_key(&self) -> bool {
        self.link_key
    }

    #[must_use]
    pub const fn key_data(&self) -> &Data {
        &self.key_data
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
