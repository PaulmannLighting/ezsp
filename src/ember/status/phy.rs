use num_derive::{FromPrimitive, ToPrimitive};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Phy {
    TxUnderflow = 0x88,
    TxIncomplete = 0x89,
    InvalidChannel = 0x8A,
    InvalidPower = 0x8B,
    TxBusy = 0x8C,
    TxCcaFail = 0x8D,
    OscillatorCheckFailed = 0x8E,
    AckReceived = 0x8F,
}

impl Display for Phy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TxUnderflow => write!(f, "TX underflow"),
            Self::TxIncomplete => write!(f, "TX incomplete"),
            Self::InvalidChannel => write!(f, "invalid channel"),
            Self::InvalidPower => write!(f, "invalid power"),
            Self::TxBusy => write!(f, "TX busy"),
            Self::TxCcaFail => write!(f, "TX CCA fail"),
            Self::OscillatorCheckFailed => write!(f, "oscillator check failed"),
            Self::AckReceived => write!(f, "ACK received"),
        }
    }
}

impl Error for Phy {}
