use crate::ember::device::Update;
use crate::ember::join::Decision;
use crate::ember::{Eui64, NodeId};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0024;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    new_node_id: NodeId,
    new_node_eui64: Eui64,
    status: u8,
    policy_decision: u8,
    parent_of_new_node_id: NodeId,
}

impl Response {
    #[must_use]
    pub fn new(
        new_node_id: NodeId,
        new_node_eui64: Eui64,
        status: Update,
        policy_decision: Decision,
        parent_of_new_node_id: NodeId,
    ) -> Self {
        Self {
            new_node_id,
            new_node_eui64,
            status: status.into(),
            policy_decision: policy_decision.into(),
            parent_of_new_node_id,
        }
    }

    #[must_use]
    pub const fn new_node_id(&self) -> NodeId {
        self.new_node_id
    }

    #[must_use]
    pub const fn new_node_eui64(&self) -> Eui64 {
        self.new_node_eui64
    }

    pub fn status(&self) -> Result<Update, u8> {
        Update::try_from(self.status)
    }

    pub fn policy_decision(&self) -> Result<Decision, u8> {
        Decision::try_from(self.policy_decision)
    }

    #[must_use]
    pub const fn parent_of_new_node_id(&self) -> NodeId {
        self.parent_of_new_node_id
    }
}
