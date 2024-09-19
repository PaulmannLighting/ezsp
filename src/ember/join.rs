use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember join decision.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Decision {
    /// Allow the node to join. The joining node should have a pre-configured key.
    ///
    /// The security data sent to it will be encrypted with that key.
    UsePreconfiguredKey = 0x00,
    /// Allow the node to join. Send the network key in-the-clear to the joining device.
    SendKeyInTheClear = 0x01,
    /// Deny join.
    DenyJoin = 0x02,
    /// Take no action.
    NoAction = 0x03,
}

impl From<Decision> for u8 {
    fn from(decision: Decision) -> Self {
        decision as Self
    }
}

impl TryFrom<u8> for Decision {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Method {
    MacAssociation = 0x00,
    NwkRejoin = 0x01,
    NwkRejoinHaveNwkKey = 0x02,
    ConfiguredNwkState = 0x03,
}

impl From<Method> for u8 {
    fn from(method: Method) -> Self {
        method as Self
    }
}

impl TryFrom<u8> for Method {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
