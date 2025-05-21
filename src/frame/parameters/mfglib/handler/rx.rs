use le_stream::derive::FromLeStream;

use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x008E;

/// A callback indicating a packet with a valid CRC has been received.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    link_quality: u8,
    rssi: i8,
    content: ByteSizedVec<u8>,
}

impl Handler {
    /// The link quality observed during the reception
    #[must_use]
    pub const fn link_quality(&self) -> u8 {
        self.link_quality
    }

    /// The energy level (in units of dBm) observed during the reception.
    #[must_use]
    pub const fn rssi(&self) -> i8 {
        self.rssi
    }

    /// The received packet (last 2 bytes are not FCS / CRC and may be discarded).
    ///
    /// Its length will be greater than 3 and less than 123.
    #[must_use]
    pub fn content(&self) -> &[u8] {
        self.content.as_slice()
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
