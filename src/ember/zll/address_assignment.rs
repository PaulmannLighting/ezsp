use crate::ember::{MulticastId, NodeId};
use le_stream::derive::{FromLeBytes, ToLeBytes};

/// ZLL address assignment data.
#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct AddressAssignment {
    node_id: NodeId,
    free_node_id_min: NodeId,
    free_node_id_max: NodeId,
    group_id_min: MulticastId,
    group_id_max: MulticastId,
    free_group_id_min: MulticastId,
    free_group_id_max: MulticastId,
}

impl AddressAssignment {
    /// Create new ZLL address assignment data.
    #[must_use]
    pub const fn new(
        node_id: NodeId,
        free_node_id_min: NodeId,
        free_node_id_max: NodeId,
        group_id_min: MulticastId,
        group_id_max: MulticastId,
        free_group_id_min: MulticastId,
        free_group_id_max: MulticastId,
    ) -> Self {
        Self {
            node_id,
            free_node_id_min,
            free_node_id_max,
            group_id_min,
            group_id_max,
            free_group_id_min,
            free_group_id_max,
        }
    }

    /// Return the relevant node id.
    #[must_use]
    pub const fn node_id_(&self) -> NodeId {
        self.node_id
    }

    /// Return the minimum free node id.
    #[must_use]
    pub const fn free_node_id_min(&self) -> NodeId {
        self.free_node_id_min
    }

    /// Return the maximum free node id.
    #[must_use]
    pub const fn free_node_id_max(&self) -> NodeId {
        self.free_node_id_max
    }

    /// Return the minimum group id.
    #[must_use]
    pub const fn group_id_min(&self) -> MulticastId {
        self.group_id_min
    }

    /// Return the maximum group id.
    #[must_use]
    pub const fn group_id_max(&self) -> MulticastId {
        self.group_id_max
    }

    /// Return the minimum free group id.
    #[must_use]
    pub const fn free_group_id_min(&self) -> MulticastId {
        self.free_group_id_min
    }

    /// Return the maximum free group id.
    #[must_use]
    pub const fn free_group_id_max(&self) -> MulticastId {
        self.free_group_id_max
    }
}
