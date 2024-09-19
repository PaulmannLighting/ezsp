use crate::ember::Eui64;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0075;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    address: Eui64,
    link_key: bool,
}

impl Command {
    #[must_use]
    pub const fn new(address: Eui64, link_key: bool) -> Self {
        Self { address, link_key }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    index: u8,
}

impl Response {
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
