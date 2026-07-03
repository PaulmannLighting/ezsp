//! Parameters for the [`Binding::binding_is_active`](crate::Binding::is_active) command.

crate::frame::parameters::frame!(0x002E, { index: u8 }, { active: bool });

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Response {
    /// True if the binding table entry is active, false otherwise.
    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
}
