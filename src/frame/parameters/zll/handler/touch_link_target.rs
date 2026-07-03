use crate::ember::zll::Network;

crate::frame::parameters::handler!(
    0x00BB,
    { network_info: Network },
    impl {
        impl Handler {
            /// Information about the network.
            #[must_use]
            pub const fn network_info(&self) -> &Network {
                &self.network_info
            }
        }
    }
);
