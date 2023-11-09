use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool,EmberEUI64};

pub const ID: u16 = 0x0075;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    address: EmberEUI64,
    link_key: bool,
}

impl Command {
    #[must_use]
    pub const fn new(address: EmberEUI64, link_key: bool) -> Self {
        Self { address, link_key }
    }

    #[must_use]
    pub const fn address(&self) -> EmberEUI64 {
        self.address
    }


    #[must_use]
    pub const fn link_key(&self) -> bool {
        self.link_key
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
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
