use num_derive::{FromPrimitive, ToPrimitive};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Flash {
    WriteInhibited = 0x46,
    VerifyFailed = 0x47,
    ProgFail = 0x4B,
    EraseFail = 0x4C,
}

impl Display for Flash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WriteInhibited => write!(f, "write inhibited"),
            Self::VerifyFailed => write!(f, "verify failed"),
            Self::ProgFail => write!(f, "programming failed"),
            Self::EraseFail => write!(f, "erasing failed"),
        }
    }
}

impl Error for Flash {}
