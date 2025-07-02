//! Radio parameters.

use le_stream::derive::{FromLeStream, ToLeStream};

/// Radio parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Parameters {
    tx_power: i8,
    page: u8,
    channel: u8,
}

impl Parameters {
    /// Create new radio parameters.
    #[must_use]
    pub const fn new(tx_power: i8, page: u8, channel: u8) -> Self {
        Self {
            tx_power,
            page,
            channel,
        }
    }

    /// Return power setting, in dBm.
    #[must_use]
    pub const fn tx_power(&self) -> i8 {
        self.tx_power
    }

    /// Return radio page.
    #[must_use]
    pub const fn page(&self) -> u8 {
        self.page
    }

    /// Return radio channel.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}
