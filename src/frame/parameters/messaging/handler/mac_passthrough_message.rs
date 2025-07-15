use le_stream::Prefixed;
use le_stream::derive::FromLeStream;

use crate::ember::mac::PassThroughType;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0097;

/// A callback invoked by the `EmberZNet` stack when a MAC passthrough message is received.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    message_type: u8,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    message: Prefixed<u8, ByteSizedVec<u8>>,
}

impl Handler {
    /// The type of MAC passthrough message received.
    ///
    /// # Errors
    ///
    /// Returns an error if the message type is not a valid [`PassThroughType`].
    pub fn message_type(&self) -> Result<PassThroughType, u8> {
        PassThroughType::try_from(self.message_type)
    }

    /// The link quality from the node that last relayed the message.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// The energy level (in units of dBm) observed during reception.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }

    /// The raw message that was received.
    #[must_use]
    pub fn message(&self) -> &[u8] {
        self.message.as_slice()
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
