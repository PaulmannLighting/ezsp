use std::error::Error;
use std::fmt::{Display, Formatter};

use super::values::Values;
use num_traits::FromPrimitive;

/// Ember PHY status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Phy {
    /// The transmit hardware buffer underflowed.
    TxUnderflow,
    /// The transmit hardware did not finish transmitting a packet.
    TxIncomplete,
    /// An unsupported channel setting was specified.
    InvalidChannel,
    /// An unsupported power setting was specified.
    InvalidPower,
    /// The packet cannot be transmitted because the physical MAC layer is currently transmitting a packet.
    ///
    /// (This is used for the MAC backoff algorithm.)
    TxBusy,
    /// The transmit attempt failed because all CCA attempts indicated that the channel was busy.
    TxCcaFail,
    /// The software installed on the hardware doesn't recognize the hardware radio type.
    OscillatorCheckFailed,
    /// The expected ACK was received after the last transmission.
    AckReceived,
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

impl From<Phy> for Values {
    fn from(phy: Phy) -> Self {
        match phy {
            Phy::TxUnderflow => Self::PhyTxUnderflow,
            Phy::TxIncomplete => Self::PhyTxIncomplete,
            Phy::InvalidChannel => Self::PhyInvalidChannel,
            Phy::InvalidPower => Self::PhyInvalidPower,
            Phy::TxBusy => Self::PhyTxBusy,
            Phy::TxCcaFail => Self::PhyTxCcaFail,
            Phy::OscillatorCheckFailed => Self::PhyOscillatorCheckFailed,
            Phy::AckReceived => Self::PhyAckReceived,
        }
    }
}

impl From<Phy> for u8 {
    fn from(phy: Phy) -> Self {
        Values::from(phy).into()
    }
}

impl TryFrom<Values> for Phy {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::PhyTxUnderflow => Ok(Self::TxUnderflow),
            Values::PhyTxIncomplete => Ok(Self::TxIncomplete),
            Values::PhyInvalidChannel => Ok(Self::InvalidChannel),
            Values::PhyInvalidPower => Ok(Self::InvalidPower),
            Values::PhyTxBusy => Ok(Self::TxBusy),
            Values::PhyTxCcaFail => Ok(Self::TxCcaFail),
            Values::PhyOscillatorCheckFailed => Ok(Self::OscillatorCheckFailed),
            Values::PhyAckReceived => Ok(Self::AckReceived),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Phy {
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
