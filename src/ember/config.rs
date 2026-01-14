//! Configuration values for the Ember stack.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Transmit Power Mode configuration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, FromPrimitive)]
#[repr(u16)]
pub enum TxPowerMode {
    /// Normal transmit power mode.
    #[default]
    Default = 0x00,
    /// Boost transmit power mode.
    Boost = 0x01,
    /// Alternate transmit power mode.
    Alternate = 0x02,
    /// Boost and Alternate transmit power mode.
    BoostAndAlternate = 0x03,
}

impl From<TxPowerMode> for u16 {
    fn from(mode: TxPowerMode) -> Self {
        mode as Self
    }
}

impl TryFrom<u16> for TxPowerMode {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
