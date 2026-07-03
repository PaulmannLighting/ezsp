use crate::ember::device::Update;
use crate::ember::join::Decision;
use crate::ember::{Eui64, NodeId};

crate::frame::parameters::handler!(
    0x0024,
    { new_node_id: NodeId, new_node_eui64: Eui64, status: u8, policy_decision: u8, parent_of_new_node_id: NodeId },
    impl {
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

            /// A [`Decision`] reflecting the decision made.
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
    }
);
