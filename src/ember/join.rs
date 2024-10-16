//! Ember join decision and method.

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

/// Ember join method.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Method {
    /// Normally devices use MAC Association to join a network,
    /// which respects the "permit joining" flag in the MAC Beacon.
    ///
    /// This value should be used by default.
    #[default]
    MacAssociation = 0x00,
    /// For those networks where the "permit joining" flag is never turned on,
    /// they will need to use a ZigBee NWK Rejoin.
    ///
    /// This value causes the rejoin to be sent without NWK security and the Trust Center
    /// will be asked to send the NWK key to the device.
    /// The NWK key sent to the device can be encrypted with the device's corresponding
    /// Trust Center link key.
    /// That is determined by the [`Decision`] on the Trust Center returned by the
    /// `emberTrustCenterJoinHandler()`.
    NwkRejoin = 0x01,
    /// For those networks where the "permit joining" flag is never turned on,
    /// they will need to use a ZigBee NWK Rejoin.
    ///
    /// This value causes the rejoin to be sent without NWK security and the Trust Center
    /// will be asked to send the NWK key to the device.
    /// The NWK key sent to the device can be encrypted with the device's corresponding
    /// Trust Center link key.
    /// That is determined by the [`Decision`] on the Trust Center returned by the
    /// `emberTrustCenterJoinHandler()`.
    NwkRejoinHaveNwkKey = 0x02,
    /// For those networks where all network and security information is known ahead of time,
    /// a router device may be commissioned such that it does not need to send any messages
    /// to begin communicating on the network.
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
