use crate::ember::NodeId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x005D;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    address_table_index: u8,
    id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8, id: NodeId) -> Self {
        Self {
            address_table_index,
            id,
        }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }

    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
