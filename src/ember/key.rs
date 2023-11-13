use crate::ember::types::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

pub type Data = [u8; 16];

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Type {
    TrustCenterLinkKey = 0x01,
    CurrentNetworkKey = 0x03,
    NextNetworkKey = 0x04,
    ApplicationLinkKey = 0x05,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Bitmask {
    HasSequenceNumber = 0x00001,
    HasOutgoingFrameCounter = 0x0002,
    HasIncomingFrameCounter = 0x0004,
    HasPartnerEui64 = 0x0008,
}

impl From<Bitmask> for u16 {
    fn from(struct_bitmask: Bitmask) -> Self {
        struct_bitmask
            .to_u16()
            .expect("could not convert StructBitmask to u8")
    }
}

impl TryFrom<u16> for Bitmask {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Struct {
    bitmask: u16,
    typ: u8,
    key: Data,
    outgoing_frame_counter: u32,
    incoming_frame_counter: u32,
    sequence_number: u8,
    partner_eui64: Eui64,
}

impl Struct {
    #[must_use]
    pub const fn new(
        bitmask: Bitmask,
        typ: Type,
        key: Data,
        outgoing_frame_counter: u32,
        incoming_frame_counter: u32,
        sequence_number: u8,
        partner_eui64: Eui64,
    ) -> Self {
        Self {
            bitmask: bitmask.into(),
            typ: typ.into(),
            key,
            outgoing_frame_counter,
            incoming_frame_counter,
            sequence_number,
            partner_eui64,
        }
    }

    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }

    #[must_use]
    pub const fn key(&self) -> &Data {
        &self.key
    }

    #[must_use]
    pub const fn outgoing_frame_counter(&self) -> u32 {
        self.outgoing_frame_counter
    }

    #[must_use]
    pub const fn incoming_frame_counter(&self) -> u32 {
        self.incoming_frame_counter
    }

    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }

    #[must_use]
    pub const fn partner_eui64(&self) -> Eui64 {
        self.partner_eui64
    }
}
