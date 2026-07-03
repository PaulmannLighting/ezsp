//! Parameters for the [`Networking::set_power_descriptor`](crate::Networking::set_power_descriptor) command.

crate::frame::parameters::frame!(
    0x0016,
    { descriptor: u16 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(descriptor: u16) -> Self {
                Self { descriptor }
            }
        }
    },
    {}
);
