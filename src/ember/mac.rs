use num_derive::{FromPrimitive, ToPrimitive};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Mac {
    NoData = 0x31,
    JoinedNetwork = 0x32,
    BadScanDuration = 0x33,
    IncorrectScanType = 0x34,
    InvalidChannelMask = 0x35,
    CommandTransmitFailure = 0x36,
    TransmitQueueFull = 0x39,
    UnknownHeaderType = 0x3A,
    Scanning = 0x3D,
    NoAckReceived = 0x40,
    IndirectTimeout = 0x42,
}

impl Display for Mac {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoData => write!(f, "no data"),
            Self::JoinedNetwork => write!(f, "joined network"),
            Self::BadScanDuration => write!(f, "bad scan duration"),
            Self::IncorrectScanType => write!(f, "incorrect scan type"),
            Self::InvalidChannelMask => write!(f, "invalid channel mask"),
            Self::CommandTransmitFailure => write!(f, "command transmit failure"),
            Self::TransmitQueueFull => write!(f, "transmit queue full"),
            Self::UnknownHeaderType => write!(f, "unknown header type"),
            Self::Scanning => write!(f, "scanning"),
            Self::NoAckReceived => write!(f, "no ACK received"),
            Self::IndirectTimeout => write!(f, "indirect timeout"),
        }
    }
}

impl Error for Mac {}
