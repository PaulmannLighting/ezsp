use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00C3;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
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

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
