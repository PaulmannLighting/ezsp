//! Parameters for the [`SinkTable::number_of_active_entries`](crate::SinkTable::number_of_active_entries) command.

crate::frame::parameters::frame!(0x0118, {}, { number_of_entries: u8 });

impl Response {
    /// The number of active entries in the sink table.
    #[must_use]
    pub const fn number_of_entries(&self) -> u8 {
        self.number_of_entries
    }
}
