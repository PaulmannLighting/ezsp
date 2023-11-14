use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Incoming {
    Unicast = 0x00,
    UnicastReply = 0x01,
    Multicast = 0x02,
    MulticastLoopback = 0x03,
    Broadcast = 0x04,
    BroadcastLoopback = 0x05,
    ManyToOneRouteRequest = 0x06,
}

impl From<Incoming> for u8 {
    fn from(incoming: Incoming) -> Self {
        incoming.to_u8().expect("could not convert Incoming to u8")
    }
}

impl TryFrom<u8> for Incoming {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Outgoing {
    Direct = 0x00,
    ViaAddressTable = 0x01,
    ViaBinding = 0x02,
    Multicast = 0x03,
    Broadcast = 0x04,
}

impl From<Outgoing> for u8 {
    fn from(outgoing: Outgoing) -> Self {
        outgoing.to_u8().expect("could not convert Outgoing to u8")
    }
}

impl TryFrom<u8> for Outgoing {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
