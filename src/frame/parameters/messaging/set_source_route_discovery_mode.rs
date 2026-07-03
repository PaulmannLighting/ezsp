//! Parameters for the [`Messaging::set_source_route_discovery_mode`](crate::Messaging::set_source_route_discovery_mode) command.

use core::time::Duration;

use crate::types::SourceRouteDiscoveryMode;

crate::frame::parameters::frame!(
    0x005A,
    { mode: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(mode: SourceRouteDiscoveryMode) -> Self {
                Self { mode: mode.into() }
            }
        }
    },
    { remaining_time: u32 },
    impl {
        impl Response {
            /// Remaining time until next `MTORR` broadcast if the mode is on, else `None`.
            #[must_use]
            pub const fn remaining_time(&self) -> Option<Duration> {
                if self.remaining_time == u32::MAX {
                    None
                } else {
                    Some(Duration::from_millis(self.remaining_time as u64))
                }
            }
        }
    }
);
