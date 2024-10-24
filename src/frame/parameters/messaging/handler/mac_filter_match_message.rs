use le_stream::derive::FromLeStream;

use crate::ember::mac::PassThroughType;
use crate::frame::Identified;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0046;

/// A callback invoked by the `EmberZNet` stack when a raw MAC message that
/// has matched one of the application's configured MAC filters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    filter_index_match: u8,
    legacy_passthrough_type: u8,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    message: ByteSizedVec<u8>,
}

impl Handler {
    /// The index of the filter that was matched.
    #[must_use]
    pub const fn filter_index_match(&self) -> u8 {
        self.filter_index_match
    }

    /// The type of MAC passthrough message received.
    ///
    /// # Errors
    ///
    /// Returns an error if the message type is not a valid [`PassThroughType`].
    pub fn legacy_passthrough_type(&self) -> Result<PassThroughType, u8> {
        PassThroughType::try_from(self.legacy_passthrough_type)
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
        &self.message
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
