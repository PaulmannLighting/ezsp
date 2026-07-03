//! Parameters for the [`Messaging::set_extended_timeout`](crate::Messaging::set_extended_timeout) command.

use crate::ember::Eui64;

crate::frame::parameters::frame!(0x007E, { remote_eui64: Eui64, extended_timeout: bool }, {});

impl Command {
    #[must_use]
    pub const fn new(remote_eui64: Eui64, extended_timeout: bool) -> Self {
        Self {
            remote_eui64,
            extended_timeout,
        }
    }
}
