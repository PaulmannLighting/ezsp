//! Parameters for the [`Networking::id`](crate::Networking::id) command.

use crate::ember::NodeId;

crate::frame::parameters::frame!(0x0107, { child_id: NodeId }, { child_index: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(child_id: NodeId) -> Self {
        Self { child_id }
    }
}

impl Response {
    /// Returns the child index.
    #[must_use]
    pub const fn child_index(&self) -> u8 {
        self.child_index
    }
}
