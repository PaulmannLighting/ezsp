use std::error::Error;
use std::fmt::{Display, Formatter};

use num_traits::FromPrimitive;

use super::values::Values;

/// Ember serial status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Serial {
    /// Specified an invalid baud rate.
    InvalidBaudRate,
    /// Specified an invalid serial port.
    InvalidPort,
    /// Tried to send too much data.
    TxOverflow,
    /// There was not enough space to store a received character and the character was dropped.
    RxOverflow,
    /// Detected a UART framing error.
    RxFrameError,
    /// Detected a UART parity error.
    RxParityError,
    /// There is no received data to process.
    RxEmpty,
    /// The receive interrupt was not handled in time, and a character was dropped.
    RxOverrunError,
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

impl Error for Serial {}

impl From<Serial> for Values {
    fn from(serial: Serial) -> Self {
        match serial {
            Serial::InvalidBaudRate => Self::SerialInvalidBaudRate,
            Serial::InvalidPort => Self::SerialInvalidPort,
            Serial::TxOverflow => Self::SerialTxOverflow,
            Serial::RxOverflow => Self::SerialRxOverflow,
            Serial::RxFrameError => Self::SerialRxFrameError,
            Serial::RxParityError => Self::SerialRxParityError,
            Serial::RxEmpty => Self::SerialRxEmpty,
            Serial::RxOverrunError => Self::SerialRxOverrunError,
        }
    }
}

impl From<Serial> for u8 {
    fn from(serial: Serial) -> Self {
        Values::from(serial).into()
    }
}

impl TryFrom<Values> for Serial {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::SerialInvalidBaudRate => Ok(Self::InvalidBaudRate),
            Values::SerialInvalidPort => Ok(Self::InvalidPort),
            Values::SerialTxOverflow => Ok(Self::TxOverflow),
            Values::SerialRxOverflow => Ok(Self::RxOverflow),
            Values::SerialRxFrameError => Ok(Self::RxFrameError),
            Values::SerialRxParityError => Ok(Self::RxParityError),
            Values::SerialRxEmpty => Ok(Self::RxEmpty),
            Values::SerialRxOverrunError => Ok(Self::RxOverrunError),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Serial {
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
