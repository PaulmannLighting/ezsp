//! Parameters for the [`Messaging::maximum_payload_length`](crate::Messaging::maximum_payload_length) command.

crate::frame::parameters::frame!(
    0x0033,
    {},
    { aps_length: u8 },
    impl {
        impl Response {
            /// Returns the maximum payload length in bytes.
            #[must_use]
            pub const fn aps_length(&self) -> u8 {
                self.aps_length
            }
        }
    }
);
