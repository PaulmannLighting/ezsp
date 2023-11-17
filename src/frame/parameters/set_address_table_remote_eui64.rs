use crate::ember::{Eui64, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x005C;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    address_table_index: u8,
    eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8, eui64: Eui64) -> Self {
        Self {
            address_table_index,
            eui64,
        }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }

    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
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
