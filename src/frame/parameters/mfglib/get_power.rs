//! Parameters for the [`Mfglib::get_power`](crate::Mfglib::get_power) command.

crate::frame::parameters::frame!(0x008D, {}, { power: i8 });

impl Response {
    /// Returns the power level in dBm.
    #[must_use]
    pub const fn power(&self) -> i8 {
        self.power
    }
}
