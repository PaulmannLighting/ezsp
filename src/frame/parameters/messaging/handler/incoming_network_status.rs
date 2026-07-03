use crate::ember::NodeId;

crate::frame::parameters::handler!(
    0x00C4,
    { error_code: u8, target: NodeId },
    impl {
        impl Handler {
            /// One byte over-the-air error code from network status message.
            #[must_use]
            pub const fn error_code(&self) -> u8 {
                self.error_code
            }

            /// The short ID of the remote node.
            #[must_use]
            pub const fn target(&self) -> NodeId {
                self.target
            }
        }
    }
);
