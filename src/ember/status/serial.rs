use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Serial {
    InvalidBaudRate = 0x20,
    InvalidPort = 0x21,
    TxOverflow = 0x22,
    RxOverflow = 0x23,
    RxFrameError = 0x24,
    RxParityError = 0x025,
    RxEmpty = 0x26,
    RxOverrunError = 0x27,
}

impl Display for Serial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBaudRate => write!(f, "invalid baud rate"),
            Self::InvalidPort => write!(f, "invalid port"),
            Self::TxOverflow => write!(f, "TX overflow"),
            Self::RxOverflow => write!(f, "RX overflow"),
            Self::RxFrameError => write!(f, "RX frame error"),
            Self::RxParityError => write!(f, "RX parity error"),
            Self::RxEmpty => write!(f, "RX empty"),
            Self::RxOverrunError => write!(f, "RX overrun error"),
        }
    }
}

impl From<Serial> for u8 {
    fn from(serial: Serial) -> Self {
        serial as Self
    }
}

impl Error for Serial {}
