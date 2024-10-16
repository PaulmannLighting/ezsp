//! Ember MAC layer types.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember MAC pass-through type.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum PassThroughType {
    /// No MAC pass-through messages.
    None = 0x00,
    /// SE InterPAN messages.
    SeInterPan = 0x01,
    /// Legacy EmberNet messages.
    EmberNet = 0x02,
    /// Legacy EmberNet messages filtered by their source address.
    EmberNetSource = 0x04,
}

impl From<PassThroughType> for u8 {
    fn from(pass_through_type: PassThroughType) -> Self {
        pass_through_type as Self
    }
}

impl TryFrom<u8> for PassThroughType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
