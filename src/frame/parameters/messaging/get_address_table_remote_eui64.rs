use crate::ember::Eui64;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x005E;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    eui64: Eui64,
}

impl Response {
    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
