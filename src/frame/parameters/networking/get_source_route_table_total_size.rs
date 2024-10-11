use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00C3;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter<u16> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    source_route_table_total_size: u8,
}

impl Response {
    #[must_use]
    pub const fn source_route_table_total_size(&self) -> u8 {
        self.source_route_table_total_size
    }
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}
