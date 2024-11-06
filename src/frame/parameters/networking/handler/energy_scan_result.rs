use le_stream::derive::FromLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x0048;

/// Reports the result of an energy scan for a single channel.
///
/// The scan is not complete until the [`scan_complete::Handler`](super::scan_complete::Handler)
/// callback is called.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    channel: u8,
    max_rssi_value: i8,
}

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

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
