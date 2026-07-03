//! Parameters for the [`Zll::is_zll_network`](crate::Zll::is_zll_network) command.

crate::frame::parameters::frame!(
    0x00BE,
    {},
    { is_zll_network: bool },
    impl {
        impl Response {
            /// Returns whether the network is a ZLL network.
            #[must_use]
            pub const fn is_zll_network(&self) -> bool {
                self.is_zll_network
            }
        }
    }
);
