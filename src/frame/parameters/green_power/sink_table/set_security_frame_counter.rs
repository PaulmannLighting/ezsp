//! Parameters for the [`SinkTable::set_security_frame_counter`](crate::SinkTable::set_security_frame_counter) command.

crate::frame::parameters::frame!(0x00F5, { index: u8, sfc: u32 }, {});

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(index: u8, sfc: u32) -> Self {
        Self { index, sfc }
    }
}
