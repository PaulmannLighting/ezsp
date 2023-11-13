use crate::ember::{Eui64, NodeId};
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::FromLeBytes;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

// Documentation: https://docs.silabs.com/d/zigbee-stack-api/7.2.2/gp-types-h

pub const LIST_ENTRIES: usize = 2;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Address {
    sink_eui: Eui64,
    sink_node_id: NodeId,
}

impl Address {
    #[must_use]
    pub const fn new(sink_eui: Eui64, sink_node_id: NodeId) -> Self {
        Self {
            sink_eui,
            sink_node_id,
        }
    }

    #[must_use]
    pub const fn sink_eui(&self) -> Eui64 {
        self.sink_eui
    }

    #[must_use]
    pub const fn sink_node_id(&self) -> NodeId {
        self.sink_node_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Group {
    group_id: u16,
    alias: u16,
}

impl Group {
    #[must_use]
    pub const fn new(group_id: u16, alias: u16) -> Self {
        Self { group_id, alias }
    }

    #[must_use]
    pub const fn group_id(&self) -> u16 {
        self.group_id
    }

    #[must_use]
    pub const fn alias(&self) -> u16 {
        self.alias
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Type {
    FullUnicast = 0x00,
    DGroupCast = 0x01,
    GroupCast = 0x02,
    LwUnicast = 0x03,
    Unused = 0xFF,
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ.to_u8().expect("could not convert Type to u8")
    }
}

impl TryFrom<u8> for Type {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Payload {
    Unicast(Address),
    GroupCast(Group),
    GroupList(Group),
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct ListEntry {
    typ: u8,
    bytes: [u8; 10], // Size of union
}

impl ListEntry {
    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }

    pub fn payload(&self) -> Option<Payload> {
        if let Ok(typ) = self.typ() {
            match typ {
                Type::FullUnicast => Address::from_le_bytes(&mut self.bytes.iter().copied())
                    .ok()
                    .map(Payload::Unicast),

                Type::DGroupCast => Group::from_le_bytes(&mut self.bytes.iter().copied())
                    .ok()
                    .map(Payload::GroupList),
                Type::GroupCast => Group::from_le_bytes(&mut self.bytes.iter().copied())
                    .ok()
                    .map(Payload::GroupList),
                Type::LwUnicast => Address::from_le_bytes(&mut self.bytes.iter().copied())
                    .ok()
                    .map(Payload::Unicast),
                Type::Unused => None,
            }
        }

        None
    }
}
