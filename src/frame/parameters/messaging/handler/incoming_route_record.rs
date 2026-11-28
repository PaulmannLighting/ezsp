use crate::ember::{Eui64, NodeId};
use crate::types::ByteSizedVec;

#[cfg(test)]
mod tests {
    use le_stream::FromLeStream;

    use super::Handler;

    const ROUTE_RECORD_PAYLOAD: [u8; 17] = [
        0x82, 0xA2, 0x53, 0xC1, 0x01, 0x02, 0x01, 0x88, 0x17, 0x00, 0xFF, 0xB0, 0x02, 0x0F, 0x47,
        0x6B, 0x82,
    ];

    #[test]
    fn decodes_route_record_relays_as_node_ids() {
        let handler = Handler::from_le_stream_exact(ROUTE_RECORD_PAYLOAD.into_iter()).unwrap();

        assert_eq!(handler.relays(), &[0x470F, 0x826B]);
    }
}

crate::frame::parameters::handler!(
    0x0059,
    {
        source: NodeId,
        source_eui: Eui64,
        last_hop_lqi: u8,
        last_hop_rssi: i8,
        relays: ByteSizedVec<NodeId>,
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

            /// Returns the relay node IDs in their received order.
            #[must_use]
            pub fn relays(&self) -> &[NodeId] {
                self.relays.as_ref()
            }
        }
    }
);
