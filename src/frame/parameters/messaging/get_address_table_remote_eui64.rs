//! Parameters for the [`Messaging::get_address_table_remote_eui64`](crate::Messaging::get_address_table_remote_eui64) command.

use crate::ember::Eui64;

crate::frame::parameters::frame!(
    0x005E,
    { address_table_index: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(address_table_index: u8) -> Self {
                Self {
                    address_table_index,
                }
            }
        }
    },
    { eui64: Eui64 } => Messaging(messaging)::GetAddressTableRemoteEui64,
    impl {
        impl Response {
            /// Returns the EUI64.
            #[must_use]
            pub const fn eui64(&self) -> Eui64 {
                self.eui64
            }
        }
    }
);
