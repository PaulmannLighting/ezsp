//! Parameters for the [`Wwah::set_hub_connectivity`](crate::Wwah::set_hub_connectivity) command.

crate::frame::parameters::frame!(0x00E4, { connected: bool }, {});

impl Command {
    #[must_use]
    pub const fn new(connected: bool) -> Self {
        Self { connected }
    }
}
