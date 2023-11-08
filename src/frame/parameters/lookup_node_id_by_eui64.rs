use crate::types::{EmberEUI64, EmberNodeId};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0060;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    eui64: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: EmberEUI64) -> Self {
        Self { eui64 }
    }

    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
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
