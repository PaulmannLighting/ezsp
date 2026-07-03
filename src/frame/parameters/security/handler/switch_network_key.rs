crate::frame::parameters::handler!(
    0x006E,
    { sequence_number: u8 },
    impl {
        impl Handler {
            /// The sequence number of the new network key.
            #[must_use]
            pub const fn sequence_number(&self) -> u8 {
                self.sequence_number
            }
        }
    }
);
