//! Returns information about the children of the local node and the parent of the local node.

use crate::ember::{Eui64, NodeId};

crate::frame::parameters::frame!(
    0x0029,
    {},
    {
        child_count: u8,
        parent_eui64: Eui64,
        parent_node_id: NodeId,
    } => Networking(networking)::GetParentChildParameters,
    impl {
        impl Response {
            /// Returns the child count.
            #[must_use]
            pub const fn child_count(&self) -> u8 {
                self.child_count
            }

            /// Returns the parent EUI64.
            #[must_use]
            pub const fn parent_eui64(&self) -> Eui64 {
                self.parent_eui64
            }

            /// Returns the parent node ID.
            #[must_use]
            pub const fn parent_node_id(&self) -> NodeId {
                self.parent_node_id
            }
        }
    }
);
