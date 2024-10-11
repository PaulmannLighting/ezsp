use crate::ember::gp::Address;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00C0;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    addr: Address,
}

impl Command {
    #[must_use]
    pub const fn new(addr: Address) -> Self {
        Self { addr }
    }
}

impl Parameter<u16> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    index: u8,
}

impl Response {
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}
