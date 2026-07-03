//! Parameters for the [`SinkTable::lookup`](crate::SinkTable::lookup) command.

use crate::ember::gp::Address;

crate::frame::parameters::frame!(
    0x00DE,
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
    { index: u8 } => GreenPower(green_power)::SinkTable(sink_table)::Lookup,
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
