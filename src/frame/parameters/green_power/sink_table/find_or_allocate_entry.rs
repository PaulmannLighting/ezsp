//! Parameters for the [`SinkTable::find_or_allocate_entry`](crate::SinkTable::find_or_allocate_entry) command.

use crate::ember::gp::Address;

crate::frame::parameters::frame!(0x00E1, { addr: Address }, { index: u8 });

impl Command {
    #[must_use]
    pub const fn new(addr: Address) -> Self {
        Self { addr }
    }
}

impl Response {
    /// Returns the index.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}
