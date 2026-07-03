//! Parameters for the [`Networking::set_manufacturer_code`](crate::Networking::set_manufacturer_code) command.

crate::frame::parameters::frame!(0x0015, { code: u16 }, {});

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(code: u16) -> Self {
        Self { code }
    }
}
