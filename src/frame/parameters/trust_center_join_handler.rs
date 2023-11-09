use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberDeviceUpdate,EmberNodeId,EmberEUI64,EmberJoinDecision};

pub const ID: u16 = 0x0024;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    new_node_id: EmberNodeId,
    new_node_eui64: EmberEUI64,
    status: EmberDeviceUpdate,
    policy_decision: EmberJoinDecision,
    parent_of_new_node_id: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(new_node_id: EmberNodeId, new_node_eui64: EmberEUI64, status: EmberDeviceUpdate, policy_decision: EmberJoinDecision, parent_of_new_node_id: EmberNodeId) -> Self {
        Self { new_node_id, new_node_eui64, status, policy_decision, parent_of_new_node_id }
    }

    #[must_use]
    pub const fn new_node_id(&self) -> EmberNodeId {
        self.new_node_id
    }


    #[must_use]
    pub const fn new_node_eui64(&self) -> EmberEUI64 {
        self.new_node_eui64
    }


    #[must_use]
    pub const fn status(&self) -> EmberDeviceUpdate {
        self.status
    }


    #[must_use]
    pub const fn policy_decision(&self) -> EmberJoinDecision {
        self.policy_decision
    }


    #[must_use]
    pub const fn parent_of_new_node_id(&self) -> EmberNodeId {
        self.parent_of_new_node_id
    }
}
