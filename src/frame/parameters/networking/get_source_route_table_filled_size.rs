//! Parameters for the [`Networking::get_source_route_table_filled_size`](crate::Networking::get_source_route_table_filled_size) command.

crate::frame::parameters::frame!(
    0x00C2,
    {},
    { source_route_table_filled_size: u8 },
    impl {
        impl Response {
            /// The number of filled entries in the source route table.
            #[must_use]
            pub const fn source_route_table_filled_size(&self) -> u8 {
                self.source_route_table_filled_size
            }
        }
    }
);
