//! Parameters for the [`Messaging::get_address_table_remote_node_id`](crate::Messaging::get_address_table_remote_node_id) command.

use crate::ember::NodeId;

crate::frame::parameters::frame!(
    0x005F,
    { address_table_index: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(address_table_index: u8) -> Self {
                Self {
                    address_table_index,
                }
            }
        }
    },
    { node_id: NodeId },
    impl {
        impl Response {
            /// Returns the node ID.
            #[must_use]
            pub const fn node_id(&self) -> NodeId {
                self.node_id
            }
        }
    }
);
