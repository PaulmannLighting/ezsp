//! Parameters for the [`ProxyTable::process_gp_pairing`](crate::ProxyTable::process_gp_pairing) command.

use crate::ember::gp::Address;
use crate::ember::key::Data;

crate::frame::parameters::frame!(
    0x00C9,
    { options: u32, addr: Address, comm_mode: u8, sink_network_address: u16, sink_group_id: u16, assigned_alias: u16, sink_ieee_address: [u8; 8], gpd_key: Data, gpd_security_frame_counter: u32, forwarding_radius: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[expect(clippy::too_many_arguments)]
            #[must_use]
            pub const fn new(
                options: u32,
                addr: Address,
                comm_mode: u8,
                sink_network_address: u16,
                sink_group_id: u16,
                assigned_alias: u16,
                sink_ieee_address: [u8; 8],
                gpd_key: Data,
                gpd_security_frame_counter: u32,
                forwarding_radius: u8,
            ) -> Self {
                Self {
                    options,
                    addr,
                    comm_mode,
                    sink_network_address,
                    sink_group_id,
                    assigned_alias,
                    sink_ieee_address,
                    gpd_key,
                    gpd_security_frame_counter,
                    forwarding_radius,
                }
            }
        }
    },
    { gp_pairing_added: bool } => GreenPower(green_power)::ProxyTable(proxy_table)::ProcessGpPairing,
    impl {
        impl Response {
            /// Returns whether the GP pairing was added.
            #[must_use]
            pub const fn gp_pairing_added(&self) -> bool {
                self.gp_pairing_added
            }
        }
    }
);
