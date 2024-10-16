//! Neighbor table entries.

use crate::ember::types::Eui64;
use le_stream::derive::{FromLeStream, ToLeStream};

/// A neighbor table entry stores information about the
/// reliability of RF links to and from neighboring nodes.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct TableEntry {
    short_id: u16,
    average_lqi: u8,
    in_cost: u8,
    out_cost: u8,
    age: u8,
    long_id: Eui64,
}

impl TableEntry {
    /// Create a new Ember neighbor table entry.
    #[must_use]
    pub const fn new(
        short_id: u16,
        average_lqi: u8,
        in_cost: u8,
        out_cost: u8,
        age: u8,
        long_id: Eui64,
    ) -> Self {
        Self {
            short_id,
            average_lqi,
            in_cost,
            out_cost,
            age,
            long_id,
        }
    }

    /// Return the neighbor's two-byte network id.
    #[must_use]
    pub const fn short_id(&self) -> u16 {
        self.short_id
    }

    /// Return an exponentially weighted moving average of the link quality values of incoming
    /// packets from this neighbor as reported by the PHY.
    #[must_use]
    pub const fn average_lqi(&self) -> u8 {
        self.average_lqi
    }

    /// Return the incoming cost for this neighbor, computed from the average LQI.
    ///
    /// Values range from 1 for a good link to 7 for a bad link.
    #[must_use]
    pub const fn in_cost(&self) -> u8 {
        self.in_cost
    }

    /// Return the outgoing cost for this neighbor, obtained from the most recently received
    /// neighbor exchange message from the neighbor.
    ///
    /// A value of zero means that a neighbor exchange message from the neighbor
    /// has not been received recently enough, or that our id was not present in the
    /// most recently received one.
    #[must_use]
    pub const fn out_cost(&self) -> u8 {
        self.out_cost
    }

    /// Return the number of aging periods elapsed since a link status message
    /// was last received from this neighbor.
    ///
    /// The aging period is 16 seconds.
    #[must_use]
    pub const fn age(&self) -> u8 {
        self.age
    }

    /// Return the 8-byte EUI64 of the neighbor.
    #[must_use]
    pub const fn long_id(&self) -> Eui64 {
        self.long_id
    }
}
