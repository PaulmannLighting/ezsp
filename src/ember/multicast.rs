//! Ember multicast table.

use le_stream::derive::{FromLeStream, ToLeStream};

/// Ember multicast ID.
pub type Id = u16;

/// A multicast table entry indicates that a particular endpoint is a member of a particular multicast group.
///
/// Only devices with an endpoint in a multicast group will receive messages sent to that multicast group.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct TableEntry {
    multicast_id: Id,
    endpoint: u8,
    network_index: u8,
}

impl TableEntry {
    /// Create a new Ember multicast table entry.
    #[must_use]
    pub const fn new(multicast_id: Id, endpoint: u8, network_index: u8) -> Self {
        Self {
            multicast_id,
            endpoint,
            network_index,
        }
    }

    /// Return the multicast group ID.
    #[must_use]
    pub const fn multicast_id(&self) -> Id {
        self.multicast_id
    }

    /// Return the endpoint that is a member, or 0 if this entry is not in use
    /// (the ZDO is not a member of any multicast groups).
    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    /// Return the network index of the network the entry is related to.
    #[must_use]
    pub const fn network_index(&self) -> u8 {
        self.network_index
    }
}
