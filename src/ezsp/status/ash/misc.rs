use crate::ezsp::status::values::Values;
use num_traits::FromPrimitive;

/// ASH-related status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Misc {
    /// Operation not yet complete.
    InProgress,
    /// ASH protocol started.
    Started,
    /// ASH protocol connected.
    Connected,
    /// ASH protocol disconnected.
    Disconnected,
    /// Timer expired waiting for ack.
    AckTimeout,
    /// Frame in progress cancelled.
    Cancelled,
    /// Received frame out of sequence.
    OutOfSequence,
    /// Received frame with CRC error.
    BadCrc,
    /// Received frame with comm error.
    CommError,
    /// Received frame with bad ackNum.
    BadAckNum,
    /// Received frame shorter than minimum.
    TooShort,
    /// Received frame longer than maximum.
    TooLong,
    /// Received frame with illegal control byte.
    BadControl,
    /// Received frame with illegal length for its type.
    BadLength,
    /// Received ASH Ack.
    AckReceived,
    /// Sent ASH Ack.
    AckSent,
    /// Received ASH Nak.
    NakReceived,
    /// Sent ASH Nak.
    NakSent,
    /// Received ASH RST.
    RstReceived,
    /// Sent ASH RST.
    RstSent,
    /// ASH Status.
    Status,
    /// ASH TX.
    Tx,
    /// ASH RX.
    Rx,
}

impl From<Misc> for Values {
    fn from(misc: Misc) -> Self {
        match misc {
            Misc::InProgress => Self::AshInProgress,
            Misc::Started => Self::AshStarted,
            Misc::Connected => Self::AshConnected,
            Misc::Disconnected => Self::AshDisconnected,
            Misc::AckTimeout => Self::AshAckTimeout,
            Misc::Cancelled => Self::AshCancelled,
            Misc::OutOfSequence => Self::AshOutOfSequence,
            Misc::BadCrc => Self::AshBadCrc,
            Misc::CommError => Self::AshCommError,
            Misc::BadAckNum => Self::AshBadAckNum,
            Misc::TooShort => Self::AshTooShort,
            Misc::TooLong => Self::AshTooLong,
            Misc::BadControl => Self::AshBadControl,
            Misc::BadLength => Self::AshBadLength,
            Misc::AckReceived => Self::AshAckReceived,
            Misc::AckSent => Self::AshAckSent,
            Misc::NakReceived => Self::AshNakReceived,
            Misc::NakSent => Self::AshNakSent,
            Misc::RstReceived => Self::AshRstReceived,
            Misc::RstSent => Self::AshRstSent,
            Misc::Status => Self::AshStatus,
            Misc::Tx => Self::AshTx,
            Misc::Rx => Self::AshRx,
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
