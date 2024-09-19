use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

/// Ember MAC status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Mac {
    /// No pending data exists for device doing a data poll.
    NoData = 0x31,
    /// Attempt to scan when we are joined to a network.
    JoinedNetwork = 0x32,
    /// Scan duration must be 0 to 14 inclusive.
    ///
    /// Attempt was made to scan with an incorrect duration value.
    BadScanDuration = 0x33,
    /// `emberStartScan` was called with an incorrect scan type.
    IncorrectScanType = 0x34,
    /// `emberStartScan` was called with an invalid channel mask.
    InvalidChannelMask = 0x35,
    /// Failed to scan current channel because we were unable to transmit the relevant MAC command.
    CommandTransmitFailure = 0x36,
    /// The MAC transmit queue is full.
    TransmitQueueFull = 0x39,
    /// MAC header FCR error on receive.
    UnknownHeaderType = 0x3A,
    /// The MAC can't complete this task because it is scanning.
    Scanning = 0x3D,
    /// We expected to receive an ACK following the transmission,
    /// but the MAC level ACK was never received.
    NoAckReceived = 0x40,
    /// Indirect data message timed out before polled.
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

impl From<Mac> for u8 {
    fn from(mac: Mac) -> Self {
        mac as Self
    }
}

impl Error for Mac {}
