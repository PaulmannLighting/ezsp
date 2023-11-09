use crate::types::{EmberEUI64, EmberNodeId};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0029;

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
    parent_eui64: EmberEUI64,
    parent_node_id: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(
        child_count: u8,
        parent_eui64: EmberEUI64,
        parent_node_id: EmberNodeId,
    ) -> Self {
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
    pub const fn parent_eui64(&self) -> EmberEUI64 {
        self.parent_eui64
    }

    #[must_use]
    pub const fn parent_node_id(&self) -> EmberNodeId {
        self.parent_node_id
    }
}
