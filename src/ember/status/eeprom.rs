use std::error::Error;
use std::fmt::{Display, Formatter};

use super::values::Values;
use num_traits::FromPrimitive;

/// Ember EEPROM status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Eeprom {
    /// The manufacturing and stack token format in non-volatile memory is different
    /// from what the stack expects (returned at initialization).
    MfgStackVersionMismatch,
    /// The manufacturing token format in non-volatile memory is different
    /// from what the stack expects (returned at initialization).
    MfgVersionMismatch,
    /// The stack token format in non-volatile memory is different
    /// from what the stack expects (returned at initialization).
    StackVersionMismatch,
}

impl Display for Eeprom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MfgStackVersionMismatch => write!(f, "MFG stack version mismatch"),
            Self::MfgVersionMismatch => write!(f, "MFG version mismatch"),
            Self::StackVersionMismatch => write!(f, "stack version mismatch"),
        }
    }
}

impl Error for Eeprom {}

impl From<Eeprom> for Values {
    fn from(eeprom: Eeprom) -> Self {
        match eeprom {
            Eeprom::MfgStackVersionMismatch => Values::EepromMfgStackVersionMismatch,
            Eeprom::MfgVersionMismatch => Values::EepromMfgVersionMismatch,
            Eeprom::StackVersionMismatch => Values::EepromStackVersionMismatch,
        }
    }
}

impl From<Eeprom> for u8 {
    fn from(eeprom: Eeprom) -> Self {
        Values::from(eeprom).into()
    }
}

impl TryFrom<Values> for Eeprom {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::EepromMfgStackVersionMismatch => Ok(Self::MfgStackVersionMismatch),
            Values::EepromMfgVersionMismatch => Ok(Self::MfgVersionMismatch),
            Values::EepromStackVersionMismatch => Ok(Self::StackVersionMismatch),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Eeprom {
    fn from_i64(n: i64) -> Option<Self> {
        Values::from_i64(n).and_then(|value| Self::try_from(value).ok())
    }

    fn from_u8(n: u8) -> Option<Self> {
        Values::from_u8(n).and_then(|value| Self::try_from(value).ok())
    }

    fn from_u64(n: u64) -> Option<Self> {
        Values::from_u64(n).and_then(|value| Self::try_from(value).ok())
    }
}
