//! Parameters for the [`Binding::set_binding_remote_node_id`](crate::Binding::set_remote_node_id) command.

use crate::ember::NodeId;
crate::frame::parameters::frame!(
    0x0030,
    { index: u8, node_id: NodeId },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(index: u8, node_id: NodeId) -> Self {
                Self { index, node_id }
            }
        }
    },
    {}
);
