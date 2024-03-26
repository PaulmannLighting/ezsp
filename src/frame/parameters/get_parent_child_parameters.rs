use crate::ember::{Eui64, NodeId};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0029;

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
    child_count: u8,
    parent_eui64: Eui64,
    parent_node_id: NodeId,
}

impl Response {
    #[must_use]
    pub const fn new(child_count: u8, parent_eui64: Eui64, parent_node_id: NodeId) -> Self {
        Self {
            child_count,
            parent_eui64,
            parent_node_id,
        }
    }

    #[must_use]
    pub const fn child_count(&self) -> u8 {
        self.child_count
    }

    #[must_use]
    pub const fn parent_eui64(&self) -> Eui64 {
        self.parent_eui64
    }

    #[must_use]
    pub const fn parent_node_id(&self) -> NodeId {
        self.parent_node_id
    }
}
