//! Parameters for the [`Security::erase_key_table_entry`](crate::Security::erase_key_table_entry) command.

crate::frame::parameters::frame!(
    0x0076,
    { index: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(index: u8) -> Self {
                Self { index }
            }
        }
    },
    {} => Security(security)::EraseKeyTableEntry);
