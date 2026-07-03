//! Parameters for the [`Zll::get_secondary_channel_mask`](crate::Zll::get_secondary_channel_mask) command.

crate::frame::parameters::frame!(
    0x00DA,
    {},
    { zll_secondary_channel_mask: u32 } => Zll(zll)::GetSecondaryChannelMask,
    impl {
        impl Response {
            /// ZLL secondary channel mask.
            #[must_use]
            pub const fn zll_secondary_channel_mask(&self) -> u32 {
                self.zll_secondary_channel_mask
            }
        }
    }
);
