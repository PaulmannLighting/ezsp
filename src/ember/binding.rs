//! Ember binding table.

use le_stream::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::ember::types::Eui64;

/// Ember binding type.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Type {
    /// A binding that is currently not in use.
    Unused = 0x00,
    /// A unicast binding whose 64-bit identifier is the destination EUI64.
    Unicast = 0x01,
    /// A unicast binding whose 64-bit identifier is the aggregator EUI64.
    ManyToOne = 0x02,
    /// A multicast binding whose 64-bit identifier is the group  address.
    ///
    /// A multicast binding can be used to send messages to the group and to
    /// receive messages sent to the group.
    Multicast = 0x03,
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ as Self
    }
}

impl TryFrom<u8> for Type {
    type Error = u8;

    fn try_from(typ: u8) -> Result<Self, Self::Error> {
        Self::from_u8(typ).ok_or(typ)
    }
}

/// An entry in the binding table.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct TableEntry {
    typ: u8,
    local: u8,
    cluster_id: u16,
    remote: u8,
    identifier: Eui64,
    network_index: u8,
}

impl TableEntry {
    /// Create a new binding table entry.
    #[must_use]
    pub fn new(
        typ: Type,
        local: u8,
        cluster_id: u16,
        remote: u8,
        identifier: Eui64,
        network_index: u8,
    ) -> Self {
        Self {
            typ: typ.into(),
            local,
            cluster_id,
            remote,
            identifier,
            network_index,
        }
    }

    /// Return the type of binding.
    ///
    /// # Errors
    /// Returns the [`u8`] value of the type if it has an invalid value.
    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }

    /// Return the endpoint on the local node.
    #[must_use]
    pub const fn local(&self) -> u8 {
        self.local
    }

    /// Return a cluster ID that matches one from the local endpoint's simple descriptor.
    ///
    /// This cluster ID is set by the provisioning application to indicate which part an
    /// endpoint's functionality is bound to this particular remote node and is used to distinguish
    /// between unicast and multicast bindings.
    ///
    /// Note that a binding can be used to send messages with any cluster ID,
    /// not just the one listed in the binding.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Return the endpoint on the remote node (specified by identifier).
    #[must_use]
    pub const fn remote(&self) -> u8 {
        self.remote
    }

    /// Return a 64-bit identifier.
    ///
    /// This is either the destination EUI64 (for unicasts)
    /// or the 64-bit group address (for multicasts).
    #[must_use]
    pub const fn identifier(&self) -> Eui64 {
        self.identifier
    }

    /// Return the index of the network the binding belongs to.
    #[must_use]
    pub const fn network_index(&self) -> u8 {
        self.network_index
    }
}
