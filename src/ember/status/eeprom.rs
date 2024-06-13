use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Eeprom {
    MfgStackVersionMismatch = 0x04,
    MfgVersionMismatch = 0x06,
    StackVersionMismatch = 0x07,
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

impl From<Eeprom> for u8 {
    fn from(eeprom: Eeprom) -> Self {
        eeprom as Self
    }
}

impl Error for Eeprom {}
