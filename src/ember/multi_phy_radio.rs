use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Parameters {
    radio_tx_power: i8,
    radio_page: u8,
    radio_channel: u8,
}

impl Parameters {
    #[must_use]
    pub const fn new(radio_tx_power: i8, radio_page: u8, radio_channel: u8) -> Self {
        Self {
            radio_tx_power,
            radio_page,
            radio_channel,
        }
    }

    #[must_use]
    pub const fn radio_tx_power(&self) -> i8 {
        self.radio_tx_power
    }

    #[must_use]
    pub const fn radio_page(&self) -> u8 {
        self.radio_page
    }

    #[must_use]
    pub const fn radio_channel(&self) -> u8 {
        self.radio_channel
    }
}
