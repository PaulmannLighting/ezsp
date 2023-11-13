use crate::ember::types::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x005E;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    address_table_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8) -> Self {
        Self {
            address_table_index,
        }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    eui64: Eui64,
}

impl Response {
    #[must_use]
    pub const fn new(eui64: Eui64) -> Self {
        Self { eui64 }
    }

    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }
}
