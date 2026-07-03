//! Parameters for the [`Security::erase_key_table_entry`](crate::Security::erase_key_table_entry) command.

crate::frame::parameters::frame!(0x0076, { index: u8 }, {});

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}
