//! Parameters for the [`Networking::get_num_stored_beacons`](crate::Networking::get_num_stored_beacons) command.

crate::frame::parameters::frame!(
    0x0008,
    {},
    { num_beacons: u8 },
    impl {
        impl Response {
            /// The number of stored beacons.
            #[must_use]
            pub const fn num_beacons(&self) -> u8 {
                self.num_beacons
            }
        }
    }
);
