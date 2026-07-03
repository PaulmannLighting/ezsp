use crate::ember::NodeId;

crate::frame::parameters::handler!(
    0x0044,
    { child_id: NodeId, transmit_expected: Option<bool> },
    impl {
        impl Handler {
            /// The node ID of the child that is requesting data.
            #[must_use]
            pub const fn child_id(self) -> NodeId {
                self.child_id
            }

            /// True if transmit is expected, false otherwise.
            #[must_use]
            pub const fn transmit_expected(self) -> Option<bool> {
                self.transmit_expected
            }
        }
    }
);
