//! Parameters for the [`Binding::get_binding_remote_node_id`](crate::Binding::get_remote_node_id) command.

use crate::ember::{NULL_NODE_ID, NodeId};

crate::frame::parameters::frame!(0x002F, { index: u8 }, { node_id: NodeId });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Response {
    /// The short ID of the destination node or `None` if no destination is known.
    #[must_use]
    pub const fn node_id(&self) -> Option<NodeId> {
        if self.node_id == NULL_NODE_ID {
            None
        } else {
            Some(self.node_id)
        }
    }
}
