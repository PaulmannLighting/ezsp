use crate::ember::NodeId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0030;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    index: u8,
    node_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, node_id: NodeId) -> Self {
        Self { index, node_id }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }
}
