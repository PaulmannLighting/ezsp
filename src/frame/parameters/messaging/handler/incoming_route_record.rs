use crate::ember::{Eui64, NodeId};
use crate::types::ByteSizedVec;

crate::frame::parameters::handler!(
    0x0059,
    {
        source: NodeId,
        source_eui: Eui64,
        last_hop_lqi: u8,
        last_hop_rssi: i8,
        relays: ByteSizedVec<u8>,
    },
    impl {
        impl Handler {
            /// Returns the source node id.
            #[must_use]
            pub const fn source(&self) -> NodeId {
                self.source
            }

            /// Returns the source EUI64.
            #[must_use]
            pub const fn source_eui(&self) -> Eui64 {
                self.source_eui
            }

            /// Returns the last hop LQI.
            #[must_use]
            pub const fn last_hop_lqi(&self) -> u8 {
                self.last_hop_lqi
            }

            /// Returns the last hop RSSI.
            #[must_use]
            pub const fn last_hop_rssi(&self) -> i8 {
                self.last_hop_rssi
            }

            /// Returns the relays.
            #[must_use]
            pub fn relays(&self) -> &[u8] {
                self.relays.as_ref()
            }
        }
    }
);
