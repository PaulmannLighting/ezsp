//! Parameters for the [`Zll::set_additional_state`](crate::Zll::set_additional_state) command.

crate::frame::parameters::frame!(0x00D6, { state: u16 }, {});

impl Command {
    #[must_use]
    pub const fn new(state: u16) -> Self {
        Self { state }
    }
}
