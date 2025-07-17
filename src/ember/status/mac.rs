use core::error::Error;
use core::fmt::{Display, Formatter};

use num_traits::FromPrimitive;

use super::values::Values;

/// Ember MAC status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Mac {
    /// No pending data exists for device doing a data poll.
    NoData,
    /// Attempt to scan when we are joined to a network.
    JoinedNetwork,
    /// Scan duration must be 0 to 14 inclusive.
    ///
    /// Attempt was made to scan with an incorrect duration value.
    BadScanDuration,
    /// `emberStartScan` was called with an incorrect scan type.
    IncorrectScanType,
    /// `emberStartScan` was called with an invalid channel mask.
    InvalidChannelMask,
    /// Failed to scan current channel because we were unable to transmit the relevant MAC command.
    CommandTransmitFailure,
    /// The MAC transmit queue is full.
    TransmitQueueFull,
    /// MAC header FCR error on receive.
    UnknownHeaderType,
    /// The MAC can't complete this task because it is scanning.
    Scanning,
    /// We expected to receive an ACK following the transmission,
    /// but the MAC level ACK was never received.
    NoAckReceived,
    /// Indirect data message timed out before polled.
    IndirectTimeout,
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

impl From<Mac> for Values {
    fn from(mac: Mac) -> Self {
        match mac {
            Mac::NoData => Self::MacNoData,
            Mac::JoinedNetwork => Self::MacJoinedNetwork,
            Mac::BadScanDuration => Self::MacBadScanDuration,
            Mac::IncorrectScanType => Self::MacIncorrectScanType,
            Mac::InvalidChannelMask => Self::MacInvalidChannelMask,
            Mac::CommandTransmitFailure => Self::MacCommandTransmitFailure,
            Mac::TransmitQueueFull => Self::MacTransmitQueueFull,
            Mac::UnknownHeaderType => Self::MacUnknownHeaderType,
            Mac::Scanning => Self::MacScanning,
            Mac::NoAckReceived => Self::MacNoAckReceived,
            Mac::IndirectTimeout => Self::MacIndirectTimeout,
        }
    }
}

impl From<Mac> for u8 {
    fn from(mac: Mac) -> Self {
        Values::from(mac).into()
    }
}

impl TryFrom<Values> for Mac {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::MacNoData => Ok(Self::NoData),
            Values::MacJoinedNetwork => Ok(Self::JoinedNetwork),
            Values::MacBadScanDuration => Ok(Self::BadScanDuration),
            Values::MacIncorrectScanType => Ok(Self::IncorrectScanType),
            Values::MacInvalidChannelMask => Ok(Self::InvalidChannelMask),
            Values::MacCommandTransmitFailure => Ok(Self::CommandTransmitFailure),
            Values::MacTransmitQueueFull => Ok(Self::TransmitQueueFull),
            Values::MacUnknownHeaderType => Ok(Self::UnknownHeaderType),
            Values::MacScanning => Ok(Self::Scanning),
            Values::MacNoAckReceived => Ok(Self::NoAckReceived),
            Values::MacIndirectTimeout => Ok(Self::IndirectTimeout),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Mac {
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
