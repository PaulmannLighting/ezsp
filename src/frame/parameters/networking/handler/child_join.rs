use num_traits::FromPrimitive;

use crate::ember::node::Type;
use crate::ember::{Eui64, NodeId};

crate::frame::parameters::handler!(
    0x0023,
    { index: u8, joining: bool, child_id: NodeId, child_eui64: Eui64, child_type: u8 },
    impl {
        impl Handler {
            /// The index of the child of interest.
            #[must_use]
            pub const fn index(&self) -> u8 {
                self.index
            }

            /// True if the child is joining. False the child is leaving.
            #[must_use]
            pub const fn joining(&self) -> bool {
                self.joining
            }

            /// The node ID of the child.
            #[must_use]
            pub const fn child_id(&self) -> NodeId {
                self.child_id
            }

            /// The EUI64 of the child.
            #[must_use]
            pub const fn child_eui64(&self) -> Eui64 {
                self.child_eui64
            }

            /// The node type of the child.
            ///
            /// # Errors
            ///
            /// Returns an error if the type is not a valid node type.
            pub fn child_type(&self) -> Result<Type, u8> {
                Type::from_u8(self.child_type).ok_or(self.child_type)
            }
        }
    }
);
