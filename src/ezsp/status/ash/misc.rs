use crate::ezsp::status::values::Values;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Misc {
    /// Operation not yet complete.
    InProgress,
    Started,
    Connected,
    Disconnected,
    AckTimeout,
    Cancelled,
    OutOfSequence,
    BadCrc,
    CommError,
    BadAckNum,
    TooShort,
    TooLong,
    BadControl,
    BadLength,
    AckReceived,
    AckSent,
    NakReceived,
    NakSent,
    RstReceived,
    RstSent,
    Status,
    Tx,
    Rx,
}

impl From<Misc> for Values {
    fn from(misc: Misc) -> Self {
        match misc {
            Misc::InProgress => Values::AshInProgress,
            Misc::Started => Values::AshStarted,
            Misc::Connected => Values::AshConnected,
            Misc::Disconnected => Values::AshDisconnected,
            Misc::AckTimeout => Values::AshAckTimeout,
            Misc::Cancelled => Values::AshCancelled,
            Misc::OutOfSequence => Values::AshOutOfSequence,
            Misc::BadCrc => Values::AshBadCrc,
            Misc::CommError => Values::AshCommError,
            Misc::BadAckNum => Values::AshBadAckNum,
            Misc::TooShort => Values::AshTooShort,
            Misc::TooLong => Values::AshTooLong,
            Misc::BadControl => Values::AshBadControl,
            Misc::BadLength => Values::AshBadLength,
            Misc::AckReceived => Values::AshAckReceived,
            Misc::AckSent => Values::AshAckSent,
            Misc::NakReceived => Values::AshNakReceived,
            Misc::NakSent => Values::AshNakSent,
            Misc::RstReceived => Values::AshRstReceived,
            Misc::RstSent => Values::AshRstSent,
            Misc::Status => Values::AshStatus,
            Misc::Tx => Values::AshTx,
            Misc::Rx => Values::AshRx,
        }
    }
}

impl TryFrom<Values> for Misc {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::AshInProgress => Ok(Self::InProgress),
            Values::AshStarted => Ok(Self::Started),
            Values::AshConnected => Ok(Self::Connected),
            Values::AshDisconnected => Ok(Self::Disconnected),
            Values::AshAckTimeout => Ok(Self::AckTimeout),
            Values::AshCancelled => Ok(Self::Cancelled),
            Values::AshOutOfSequence => Ok(Self::OutOfSequence),
            Values::AshBadCrc => Ok(Self::BadCrc),
            Values::AshCommError => Ok(Self::CommError),
            Values::AshBadAckNum => Ok(Self::BadAckNum),
            Values::AshTooShort => Ok(Self::TooShort),
            Values::AshTooLong => Ok(Self::TooLong),
            Values::AshBadControl => Ok(Self::BadControl),
            Values::AshBadLength => Ok(Self::BadLength),
            Values::AshAckReceived => Ok(Self::AckReceived),
            Values::AshAckSent => Ok(Self::AckSent),
            Values::AshNakReceived => Ok(Self::NakReceived),
            Values::AshNakSent => Ok(Self::NakSent),
            Values::AshRstReceived => Ok(Self::RstReceived),
            Values::AshRstSent => Ok(Self::RstSent),
            Values::AshStatus => Ok(Self::Status),
            Values::AshTx => Ok(Self::Tx),
            Values::AshRx => Ok(Self::Rx),
            value => Err(value),
        }
    }
}

impl From<Misc> for u8 {
    fn from(misc: Misc) -> Self {
        Values::from(misc).into()
    }
}

impl FromPrimitive for Misc {
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
