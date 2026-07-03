use crate::ember::zll::{DeviceInfoRecord, Network};

crate::frame::parameters::handler!(
    0x00B6,
    { network_info: Network, is_device_info_null: bool, device_info: DeviceInfoRecord, last_hop_lqi: u8, last_hop_rssi: i8 },
    impl {
        impl Handler {
            /// Information about the network.
            #[must_use]
            pub const fn network_info(&self) -> &Network {
                &self.network_info
            }

            /// Device specific information.
            #[must_use]
            pub const fn device_info(&self) -> Option<&DeviceInfoRecord> {
                if self.is_device_info_null {
                    None
                } else {
                    Some(&self.device_info)
                }
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
