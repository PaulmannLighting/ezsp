use crate::ember::PanId;

crate::frame::parameters::handler!(
    0x00D2,
    { pan_id: PanId, channel: u8 },
    impl {
        impl Handler {
            /// The unused panID which has been found.
            #[must_use]
            pub const fn pan_id(&self) -> PanId {
                self.pan_id
            }

            /// The channel that the unused panID was found on.
            #[must_use]
            pub const fn channel(&self) -> u8 {
                self.channel
            }
        }
    }
);
