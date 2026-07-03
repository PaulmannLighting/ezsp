use crate::ember::Eui64;

crate::frame::parameters::handler!(
    0x0062,
    { sender_eui64: Eui64 },
    impl {
        impl Handler {
            /// The EUI64 of the sender.
            #[must_use]
            pub const fn sender_eui64(&self) -> Eui64 {
                self.sender_eui64
            }
        }
    }
);
