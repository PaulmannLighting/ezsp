//! Parameters for the [`Zll::set_radio_idle_mode`](crate::Zll::set_radio_idle_mode) command.

use crate::ember::radio::PowerMode;

crate::frame::parameters::frame!(0x00D4, { mode: u8 }, {});

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub fn new(mode: PowerMode) -> Self {
        Self { mode: mode.into() }
    }
}
