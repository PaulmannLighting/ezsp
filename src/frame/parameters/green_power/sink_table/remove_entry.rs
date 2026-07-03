//! Parameters for the [`SinkTable::remove_entry`](crate::SinkTable::remove_entry) command.

crate::frame::parameters::frame!(
    0x00E0,
    { sink_index: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(sink_index: u8) -> Self {
                Self { sink_index }
            }
        }
    },
    {} => GreenPower(green_power)::SinkTable(sink_table)::RemoveEntry);
