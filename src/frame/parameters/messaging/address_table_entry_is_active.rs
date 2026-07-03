//! Parameters for the [`Messaging::address_table_entry_is_active`](crate::Messaging::address_table_entry_is_active) command.

crate::frame::parameters::frame!(
    0x005B,
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
    { active: bool },
    impl {
        impl Response {
            /// Returns whether the entry is active.
            #[must_use]
            pub const fn active(&self) -> bool {
                self.active
            }
        }
    }
);
