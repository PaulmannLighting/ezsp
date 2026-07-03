use crate::ember::{Eui64, NodeId};

crate::frame::parameters::handler!(
    0x007D,
    { source: NodeId, long_id: Eui64, cost: u8 },
    impl {
        impl Handler {
            /// The short id of the concentrator.
            #[must_use]
            pub const fn source(&self) -> NodeId {
                self.source
            }

            /// The EUI64 of the concentrator.
            #[must_use]
            pub const fn long_id(&self) -> Eui64 {
                self.long_id
            }

            /// The path cost to the concentrator.
            ///
            /// The cost may decrease as additional route request packets for this discovery arrive,
            /// but the callback is made only once.
            #[must_use]
            pub const fn cost(&self) -> u8 {
                self.cost
            }
        }
    }
);
