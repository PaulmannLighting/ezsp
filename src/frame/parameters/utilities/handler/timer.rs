//! Timer handler parameter.

crate::frame::parameters::handler!(0x000F, { timer_id: u8 });

impl Handler {
    /// Which timer generated the callback (0 or 1).
    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }
}
