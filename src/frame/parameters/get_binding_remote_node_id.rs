use crate::types::EmberNodeId;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x002F;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    node_id: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(node_id: EmberNodeId) -> Self {
        Self { node_id }
    }

    #[must_use]
    pub const fn node_id(&self) -> EmberNodeId {
        self.node_id
    }
}
