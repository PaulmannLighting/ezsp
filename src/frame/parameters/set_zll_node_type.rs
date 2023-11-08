use crate::types::EmberNodeType;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00D5;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    node_type: EmberNodeType,
}

impl Command {
    #[must_use]
    pub const fn new(node_type: EmberNodeType) -> Self {
        Self { node_type }
    }

    #[must_use]
    pub const fn node_type(&self) -> EmberNodeType {
        self.node_type
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
