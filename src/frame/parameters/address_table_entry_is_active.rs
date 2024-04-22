use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x005B;

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
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    active: bool,
}

impl Response {
    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}
