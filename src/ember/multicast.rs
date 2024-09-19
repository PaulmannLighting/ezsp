use le_stream::derive::{FromLeBytes, ToLeBytes};

/// Ember multicast ID.
pub type Id = u16;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct TableEntry {
    multicast_id: Id,
    endpoint: u8,
    network_index: u8,
}

impl TableEntry {
    #[must_use]
    pub const fn new(multicast_id: Id, endpoint: u8, network_index: u8) -> Self {
        Self {
            multicast_id,
            endpoint,
            network_index,
        }
    }

    #[must_use]
    pub const fn multicast_id(&self) -> Id {
        self.multicast_id
    }

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    #[must_use]
    pub const fn network_index(&self) -> u8 {
        self.network_index
    }
}
