//! Parameters for the [`Networking::neighbor_count`](crate::Networking::neighbor_count) command.

crate::frame::parameters::frame!(0x007A, {}, { value: u8 });

impl Response {
    /// Returns the number of neighbors.
    #[must_use]
    pub const fn value(&self) -> u8 {
        self.value
    }
}
