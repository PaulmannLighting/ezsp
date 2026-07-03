//! Parameters for the [`Messaging::set_mac_poll_failure_wait_time`](crate::Messaging::set_mac_poll_failure_wait_time) command.

crate::frame::parameters::frame!(
    0x00F4,
    { wait_before_retry_interval_ms: u8 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(wait_before_retry_interval_ms: u8) -> Self {
                Self {
                    wait_before_retry_interval_ms,
                }
            }
        }
    },
    {}
);
