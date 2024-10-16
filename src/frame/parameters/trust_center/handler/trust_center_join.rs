use le_stream::derive::FromLeStream;

use crate::ember::device::Update;
use crate::ember::join::Decision;
use crate::ember::{Eui64, NodeId};
use crate::frame::Parameter;

const ID: u16 = 0x0024;

/// The NCP used the trust center behavior policy to decide
/// whether to allow a new node to join the network.
///
/// The Host cannot change the current decision, but it can change the policy
/// for future decisions using the setPolicy command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    new_node_id: NodeId,
    new_node_eui64: Eui64,
    status: u8,
    policy_decision: u8,
    parent_of_new_node_id: NodeId,
}

impl Handler {
    /// The Node Id of the node whose status changed
    #[must_use]
    pub const fn new_node_id(&self) -> NodeId {
        self.new_node_id
    }

    /// The EUI64 of the node whose status changed.
    #[must_use]
    pub const fn new_node_eui64(&self) -> Eui64 {
        self.new_node_eui64
    }

    /// The status of the node: Secure Join/Rejoin, Unsecure Join/Rejoin, Device left.
    ///
    /// # Errors
    ///
    /// Returns an error if the status is invalid.
    pub fn status(&self) -> Result<Update, u8> {
        Update::try_from(self.status)
    }

    /// An EmberJoinDecision reflecting the decision made.
    ///
    /// # Errors
    ///
    /// Returns an error if the policy decision is invalid.
    pub fn policy_decision(&self) -> Result<Decision, u8> {
        Decision::try_from(self.policy_decision)
    }

    /// The parent of the node whose status has changed.
    #[must_use]
    pub const fn parent_of_new_node_id(&self) -> NodeId {
        self.parent_of_new_node_id
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
