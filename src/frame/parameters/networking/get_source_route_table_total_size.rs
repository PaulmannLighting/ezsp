//! Parameters for the [`Networking::get_source_route_table_total_size`](crate::Networking::get_source_route_table_total_size) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00C3;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    source_route_table_total_size: u8,
}

impl Response {
    /// The total size of the source route table.
    #[must_use]
    pub const fn source_route_table_total_size(&self) -> u8 {
        self.source_route_table_total_size
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
