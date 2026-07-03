//! Parameters for the [`TokenInterface::token_factory_reset`](crate::TokenInterface::token_factory_reset) command.

crate::frame::parameters::frame!(
    0x0077,
    { exclude_outgoing_fc: bool, exclude_boot_counter: bool },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(exclude_outgoing_fc: bool, exclude_boot_counter: bool) -> Self {
                Self {
                    exclude_outgoing_fc,
                    exclude_boot_counter,
                }
            }
        }
    },
    {} => TokenInterface(token_interface)::TokenFactoryReset);
