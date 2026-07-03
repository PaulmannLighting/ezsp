//! Parameters for the [`Messaging::lookup_node_id_by_eui64`](crate::Messaging::lookup_node_id_by_eui64) command.

use crate::ember::{Eui64, NodeId};

crate::frame::parameters::frame!(
    0x0060,
    { eui64: Eui64 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(eui64: Eui64) -> Self {
                Self { eui64 }
            }
        }
    },
    { node_id: NodeId } => Messaging(messaging)::LookupNodeIdByEui64,
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
