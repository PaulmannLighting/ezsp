//! Parameters for the [`Zll::get_primary_channel_mask`](crate::Zll::get_primary_channel_mask) command.

crate::frame::parameters::frame!(
    0x00D9,
    {},
    { zll_primary_channel_mask: u32 },
    impl {
        impl Response {
            /// ZLL primary channel mask.
            #[must_use]
            pub const fn zll_primary_channel_mask(&self) -> u32 {
                self.zll_primary_channel_mask
            }
        }
    }
);
