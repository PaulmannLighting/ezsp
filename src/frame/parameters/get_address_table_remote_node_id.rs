use crate::ember::NodeId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x005F;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    node_id: NodeId,
}

impl Response {
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }
}
