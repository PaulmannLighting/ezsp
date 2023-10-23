use num_derive::{FromPrimitive, ToPrimitive};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Application {
    Error0 = 0xF0,
    Error1 = 0xF1,
    Error2 = 0xF2,
    Error3 = 0xF3,
    Error4 = 0xF4,
    Error5 = 0xF5,
    Error6 = 0xF6,
    Error7 = 0xF7,
    Error8 = 0xF8,
    Error9 = 0xF9,
    Error10 = 0xFA,
    Error11 = 0xFB,
    Error12 = 0xFC,
    Error13 = 0xFD,
    Error14 = 0xFE,
    Error15 = 0xFF,
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error0 => write!(f, "error #0"),
            Self::Error1 => write!(f, "error #1"),
            Self::Error2 => write!(f, "error #2"),
            Self::Error3 => write!(f, "error #3"),
            Self::Error4 => write!(f, "error #4"),
            Self::Error5 => write!(f, "error #5"),
            Self::Error6 => write!(f, "error #6"),
            Self::Error7 => write!(f, "error #7"),
            Self::Error8 => write!(f, "error #8"),
            Self::Error9 => write!(f, "error #9"),
            Self::Error10 => write!(f, "error #10"),
            Self::Error11 => write!(f, "error #11"),
            Self::Error12 => write!(f, "error #12"),
            Self::Error13 => write!(f, "error #13"),
            Self::Error14 => write!(f, "error #14"),
            Self::Error15 => write!(f, "error #15"),
        }
    }
}

impl Error for Application {}
