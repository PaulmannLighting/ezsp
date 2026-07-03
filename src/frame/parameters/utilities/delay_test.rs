//! Parameters for the [`Utilities::delay_test`](crate::Utilities::delay_test) command.

crate::frame::parameters::frame!(
    0x009D,
    { delay: u16 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(delay: u16) -> Self {
                Self { delay }
            }
        }
    },
    {}
);
