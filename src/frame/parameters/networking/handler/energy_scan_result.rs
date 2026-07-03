crate::frame::parameters::handler!(
    0x0048,
    { channel: u8, max_rssi_value: i8 },
    impl {
        impl Handler {
            /// The 802.15.4 channel number that was scanned.
            #[must_use]
            pub const fn channel(&self) -> u8 {
                self.channel
            }

            /// The maximum RSSI value found on the channel.
            #[must_use]
            pub const fn max_rssi_value(&self) -> i8 {
                self.max_rssi_value
            }
        }
    }
);
