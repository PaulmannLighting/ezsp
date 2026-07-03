//! Parameters for the [`Mfglib::get_channel`](crate::Mfglib::get_channel) command.

crate::frame::parameters::frame!(
    0x008B,
    {},
    { channel: u8 } => MfgLib(mfglib)::GetChannel,
    impl {
        impl Response {
            /// Returns the channel.
            #[must_use]
            pub const fn channel(&self) -> u8 {
                self.channel
            }
        }
    }
);
