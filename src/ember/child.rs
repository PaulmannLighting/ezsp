//! Child node data structure.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::node::Type;
use crate::ember::types::{Eui64, NodeId};

/// A structure containing a child node's data.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Data {
    eui64: Eui64,
    typ: u8,
    id: NodeId,
    phy: u8,
    power: u8,
    timeout: u8,
}

impl Data {
    /// Create a new child data structure.
    #[must_use]
    pub fn new(eui64: Eui64, typ: Type, id: NodeId, phy: u8, power: u8, timeout: u8) -> Self {
        Self {
            eui64,
            typ: typ.into(),
            id,
            phy,
            power,
            timeout,
        }
    }

    /// Return the EUI64 of the child.
    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    /// Return the node type of the child.
    #[must_use]
    pub fn typ(&self) -> Option<Type> {
        Type::from_u8(self.typ)
    }

    /// Return the short address of the child.
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }

    /// Return the phy of the child.
    #[must_use]
    pub const fn phy(&self) -> u8 {
        self.phy
    }

    /// Return the power of the child.
    #[must_use]
    pub const fn power(&self) -> u8 {
        self.power
    }

    /// Return the timeout of the child.
    #[must_use]
    pub const fn timeout(&self) -> u8 {
        self.timeout
    }
}
