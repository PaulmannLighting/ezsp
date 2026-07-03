//! Parameters for the [`Wwah::set_long_uptime`](crate::Wwah::set_long_uptime) command.

crate::frame::parameters::frame!(0x00E3, { has_long_up_time: bool }, {});

impl Command {
    #[must_use]
    pub const fn new(has_long_up_time: bool) -> Self {
        Self { has_long_up_time }
    }
}
