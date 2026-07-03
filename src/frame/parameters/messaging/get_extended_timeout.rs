//! Parameters for the [`Messaging::get_extended_timeout`](crate::Messaging::get_extended_timeout) command.

use crate::ember::Eui64;

crate::frame::parameters::frame!(
    0x007F,
    { remote_eui64: Eui64 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(remote_eui64: Eui64) -> Self {
                Self { remote_eui64 }
            }
        }
    },
    { extended_timeout: bool } => Messaging(messaging)::GetExtendedTimeout,
    impl {
        impl Response {
            /// Returns whether the extended timeout is enabled.
            #[must_use]
            pub const fn extended_timeout(&self) -> bool {
                self.extended_timeout
            }
        }
    }
);
