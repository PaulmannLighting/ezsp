use crate::ember::zll::AddressAssignment;

crate::frame::parameters::handler!(
    0x00B8,
    { address_info: AddressAssignment, last_hop_lqi: u8, last_hop_rssi: i8 },
    impl {
        impl Handler {
            /// Address assignment information.
            #[must_use]
            pub const fn address_info(&self) -> &AddressAssignment {
                &self.address_info
            }

            /// The link quality from the node that last relayed the message.
            #[must_use]
            pub const fn last_hop_lqi(&self) -> u8 {
                self.last_hop_lqi
            }

            /// The energy level (in units of dBm) observed during reception.
            #[must_use]
            pub const fn last_hop_rssi(&self) -> i8 {
                self.last_hop_rssi
            }
        }
    }
);
