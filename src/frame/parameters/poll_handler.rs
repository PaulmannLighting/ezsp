use crate::types::{bool, EmberNodeId};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0044;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    child_id: EmberNodeId,
    transmit_expected: bool,
}

impl Response {
    #[must_use]
    pub const fn new(child_id: EmberNodeId, transmit_expected: bool) -> Self {
        Self {
            child_id,
            transmit_expected,
        }
    }

    #[must_use]
    pub const fn child_id(&self) -> EmberNodeId {
        self.child_id
    }

    #[must_use]
    pub const fn transmit_expected(&self) -> bool {
        self.transmit_expected
    }
}
