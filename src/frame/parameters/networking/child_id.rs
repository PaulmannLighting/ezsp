//! Parameters for the [`Networking::child_id`](crate::Networking::child_id) command.

use crate::ember::{NULL_NODE_ID, NodeId};

crate::frame::parameters::frame!(0x0106, { child_index: u8 }, { child_id: NodeId });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(child_index: u8) -> Self {
        Self { child_index }
    }
}

impl Response {
    /// The node ID of the child or `None` if there isn't a child at the `child_index` specified.
    #[must_use]
    pub const fn child_id(&self) -> Option<NodeId> {
        if self.child_id == NULL_NODE_ID {
            None
        } else {
            Some(self.child_id)
        }
    }
}
