use num_derive::{FromPrimitive, ToPrimitive};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
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

impl Error for Eeprom {}
