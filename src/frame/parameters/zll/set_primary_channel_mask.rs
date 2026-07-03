//! Parameters for the [`Zll::set_primary_channel_mask`](crate::Zll::set_primary_channel_mask) command.

crate::frame::parameters::frame!(0x00DB, { zll_primary_channel_mask: u32 }, {});

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(zll_primary_channel_mask: u32) -> Self {
        Self {
            zll_primary_channel_mask,
        }
    }
}
