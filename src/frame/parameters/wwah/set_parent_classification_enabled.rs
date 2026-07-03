//! Parameters for the [`Wwah::set_parent_classification_enabled`](crate::Wwah::set_parent_classification_enabled) command.

crate::frame::parameters::frame!(
    0x00E7,
    { enabled: bool },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(enabled: bool) -> Self {
                Self { enabled }
            }
        }
    },
    {}
);
