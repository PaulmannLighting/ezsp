use num_derive::{FromPrimitive, ToPrimitive};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum SimEeprom {
    ErasePageGreen = 0x43,
    ErasePageRed = 0x44,
    Full = 0x45,
    Init1Failed = 0x48,
    Init2Failed = 0x49,
    Init3Failed = 0x4A,
}

impl Display for SimEeprom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErasePageGreen => write!(f, "erase page green"),
            Self::ErasePageRed => write!(f, "erase page red"),
            Self::Full => write!(f, "full"),
            Self::Init1Failed => write!(f, "init #1 failed"),
            Self::Init2Failed => write!(f, "init #2 failed"),
            Self::Init3Failed => write!(f, "init #3 failed"),
        }
    }
}

impl Error for SimEeprom {}
