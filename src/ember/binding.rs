use crate::ember::types::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Type {
    Unused = 0x00,
    Unicast = 0x01,
    ManyToOne = 0x02,
    Multicast = 0x03,
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ.to_u8().expect("could not convert Type to u8")
    }
}

impl TryFrom<u8> for Type {
    type Error = u8;

    fn try_from(typ: u8) -> Result<Self, Self::Error> {
        Self::from_u8(typ).ok_or(typ)
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct TableEntry {
    typ: u8,
    local: u8,
    cluster_id: u16,
    remote: u8,
    identifier: Eui64,
    network_index: u8,
}

impl TableEntry {
    pub const fn new(
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

    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }

    #[must_use]
    pub const fn local(&self) -> u8 {
        self.local
    }

    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    #[must_use]
    pub const fn remote(&self) -> u8 {
        self.remote
    }

    #[must_use]
    pub const fn identifier(&self) -> Eui64 {
        self.identifier
    }

    #[must_use]
    pub const fn network_index(&self) -> u8 {
        self.network_index
    }
}
