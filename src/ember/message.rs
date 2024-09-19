use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember incoming message type.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Incoming {
    /// Unicast.
    Unicast = 0x00,
    /// Unicast reply.
    UnicastReply = 0x01,
    /// Multicast.
    Multicast = 0x02,
    /// Multicast sent by the local device.
    MulticastLoopback = 0x03,
    /// Broadcast.
    Broadcast = 0x04,
    /// Broadcast sent by the local device.
    BroadcastLoopback = 0x05,
    /// Many to one route request.
    ManyToOneRouteRequest = 0x06,
}

impl From<Incoming> for u8 {
    fn from(incoming: Incoming) -> Self {
        incoming as Self
    }
}

impl TryFrom<u8> for Incoming {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Outgoing {
    Direct = 0x00,
    ViaAddressTable = 0x01,
    ViaBinding = 0x02,
    Multicast = 0x03,
    Broadcast = 0x04,
}

impl From<Outgoing> for u8 {
    fn from(outgoing: Outgoing) -> Self {
        outgoing as Self
    }
}

impl TryFrom<u8> for Outgoing {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
