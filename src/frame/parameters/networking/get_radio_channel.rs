//! Parameters for the [`Networking::get_radio_channel`](crate::Networking::get_radio_channel) command.

crate::frame::parameters::frame!(
    0x00FF,
    {},
    { channel: u8 },
    impl {
        impl Response {
            /// Returns the radio channel.
            #[must_use]
            pub const fn channel(&self) -> u8 {
                self.channel
            }
        }
    }
);
