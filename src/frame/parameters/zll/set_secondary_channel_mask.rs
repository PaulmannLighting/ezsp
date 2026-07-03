//! Parameters for the [`Zll::set_secondary_channel_mask`](crate::Zll::set_secondary_channel_mask) command.

crate::frame::parameters::frame!(
    0x00DC,
    { zll_secondary_channel_mask: u32 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(zll_secondary_channel_mask: u32) -> Self {
                Self {
                    zll_secondary_channel_mask,
                }
            }
        }
    },
    {}
);
