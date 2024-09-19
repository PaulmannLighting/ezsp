use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

/// Ember PHY status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Phy {
    /// The transmit hardware buffer underflowed.
    TxUnderflow = 0x88,
    /// The transmit hardware did not finish transmitting a packet.
    TxIncomplete = 0x89,
    /// An unsupported channel setting was specified.
    InvalidChannel = 0x8A,
    /// An unsupported power setting was specified.
    InvalidPower = 0x8B,
    /// The packet cannot be transmitted because the physical MAC layer is currently transmitting a packet.
    ///
    /// (This is used for the MAC backoff algorithm.)
    TxBusy = 0x8C,
    /// The transmit attempt failed because all CCA attempts indicated that the channel was busy.
    TxCcaFail = 0x8D,
    /// The software installed on the hardware doesn't recognize the hardware radio type.
    OscillatorCheckFailed = 0x8E,
    /// The expected ACK was received after the last transmission.
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

impl From<Phy> for u8 {
    fn from(phy: Phy) -> Self {
        phy as Self
    }
}

impl Error for Phy {}
