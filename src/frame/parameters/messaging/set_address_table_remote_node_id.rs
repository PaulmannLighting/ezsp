//! Parameters for the [`Messaging::set_address_table_remote_node_id`](crate::Messaging::set_address_table_remote_node_id) command.

use crate::ember::NodeId;

crate::frame::parameters::frame!(0x005D, { address_table_index: u8, id: NodeId }, {});

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(address_table_index: u8, id: NodeId) -> Self {
        Self {
            address_table_index,
            id,
        }
    }
}
