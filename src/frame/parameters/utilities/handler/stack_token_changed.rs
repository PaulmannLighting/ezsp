//! Stack Token Changed Handler

crate::frame::parameters::handler!(
    0x000D,
    { token_address: u16 },
    impl {
        impl Handler {
            /// The address of the stack token that has changed.
            #[must_use]
            pub const fn token_address(&self) -> u16 {
                self.token_address
            }
        }
    }
);
