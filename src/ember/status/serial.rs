use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

/// Ember serial status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Serial {
    /// Specified an invalid baud rate.
    InvalidBaudRate = 0x20,
    /// Specified an invalid serial port.
    InvalidPort = 0x21,
    /// Tried to send too much data.
    TxOverflow = 0x22,
    /// There was not enough space to store a received character and the character was dropped.
    RxOverflow = 0x23,
    /// Detected a UART framing error.
    RxFrameError = 0x24,
    /// Detected a UART parity error.
    RxParityError = 0x025,
    /// There is no received data to process.
    RxEmpty = 0x26,
    /// The receive interrupt was not handled in time, and a character was dropped.
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
