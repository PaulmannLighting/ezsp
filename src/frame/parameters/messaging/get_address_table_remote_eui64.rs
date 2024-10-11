use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x005E;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    eui64: Eui64,
}

impl Response {
    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}
