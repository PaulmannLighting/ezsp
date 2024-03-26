use crate::ember::gp::Address;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00E1;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    addr: Address,
}

impl Command {
    #[must_use]
    pub const fn new(addr: Address) -> Self {
        Self { addr }
    }

    #[must_use]
    pub const fn addr(&self) -> &Address {
        &self.addr
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    index: u8,
}

impl Response {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}
