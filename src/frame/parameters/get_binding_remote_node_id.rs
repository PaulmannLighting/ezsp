use crate::ember::types::NodeId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x002F;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
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
