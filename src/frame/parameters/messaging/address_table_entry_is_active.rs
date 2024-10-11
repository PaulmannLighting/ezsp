use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x005B;

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
    active: bool,
}

impl Response {
    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}
