//! Parameters for the [`Networking::get_source_route_table_total_size`](crate::Networking::get_source_route_table_total_size) command.

crate::frame::parameters::frame!(
    0x00C3,
    {},
    { source_route_table_total_size: u8 } => Networking(networking)::GetSourceRouteTableTotalSize,
    impl {
        impl Response {
            /// The total size of the source route table.
            #[must_use]
            pub const fn source_route_table_total_size(&self) -> u8 {
                self.source_route_table_total_size
            }
        }
    }
);
