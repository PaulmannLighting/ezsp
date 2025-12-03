//! Parameters for the [`Networking::get_source_route_table_filled_size`](crate::Networking::get_source_route_table_filled_size) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00C2;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    source_route_table_filled_size: u8,
}

impl Response {
    /// The number of filled entries in the source route table.
    #[must_use]
    pub const fn source_route_table_filled_size(&self) -> u8 {
        self.source_route_table_filled_size
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
