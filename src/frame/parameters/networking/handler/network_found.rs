use crate::ember::zigbee::Network;

crate::frame::parameters::handler!(
    0x001B,
    { network_found: Network, last_hop_lqi: u8, last_hop_rssi: i8 },
    impl {
        impl Handler {
            /// The parameters associated with the network found.
            #[must_use]
            pub const fn network_found(&self) -> &Network {
                &self.network_found
            }

            /// The link quality from the node that generated this beacon.
            #[must_use]
            pub const fn last_hop_lqi(&self) -> u8 {
                self.last_hop_lqi
            }

            /// The energy level (in units of dBm) observed during the reception.
            #[must_use]
            pub const fn last_hop_rssi(&self) -> i8 {
                self.last_hop_rssi
            }
        }
    }
);
