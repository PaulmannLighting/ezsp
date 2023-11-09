use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00C3;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    source_route_table_total_size: u8,
}

impl Response {
    #[must_use]
    pub const fn new(source_route_table_total_size: u8) -> Self {
        Self {
            source_route_table_total_size,
        }
    }

    #[must_use]
    pub const fn source_route_table_total_size(&self) -> u8 {
        self.source_route_table_total_size
    }
}
