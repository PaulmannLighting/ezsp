use crate::ember::gp::Address;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00DE;

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

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
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

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}
