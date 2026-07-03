use crate::ember::NodeId;

crate::frame::parameters::handler!(
    0x007C,
    { id: NodeId },
    impl {
        impl Handler {
            /// The short id for which a conflict was detected.
            #[must_use]
            pub const fn id(&self) -> NodeId {
                self.id
            }
        }
    }
);
