//! Parameters for the [`Networking::get_logical_channel`](crate::Networking::get_logical_channel) command.

crate::frame::parameters::frame!(0x00BA, {}, { logical_channel: u8 });

impl Response {
    /// Returns the logical channel.
    #[must_use]
    pub const fn logical_channel(&self) -> u8 {
        self.logical_channel
    }
}
