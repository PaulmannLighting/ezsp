//! Parameters for the [`ProxyTable::lookup`](crate::ProxyTable::lookup) command.

use crate::ember::gp::Address;

crate::frame::parameters::frame!(
    0x00C0,
    { addr: Address },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(addr: Address) -> Self {
                Self { addr }
            }
        }
    },
    { index: u8 } => GreenPower(green_power)::ProxyTable(proxy_table)::Lookup,
    impl {
        impl Response {
            /// Returns the index.
            #[must_use]
            pub const fn index(&self) -> u8 {
                self.index
            }
        }
    }
);
