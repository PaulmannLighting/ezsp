use num_traits::FromPrimitive;

use crate::ezsp::status::values::Values;
pub use error::Error;
pub use misc::Misc;

mod error;
mod misc;

/// Status related to ASH.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Ash {
    /// Errors.
    Error(Error),
    /// Miscellaneous status.
    Misc(Misc),
}

impl From<Ash> for Values {
    fn from(ash: Ash) -> Self {
        match ash {
            Ash::Error(error) => error.into(),
            Ash::Misc(misc) => misc.into(),
        }
    }
}

impl TryFrom<Values> for Ash {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, <Self as TryFrom<Values>>::Error> {
        match value {
            Values::AshNcpFatalError => Ok(Self::Error(Error::NcpFatalError)),
            Values::AshErrorVersion => Ok(Self::Error(Error::Version)),
            Values::AshErrorTimeouts => Ok(Self::Error(Error::Timeouts)),
            Values::AshErrorResetFail => Ok(Self::Error(Error::ResetFail)),
            Values::AshErrorNcpReset => Ok(Self::Error(Error::NcpReset)),
            Values::AshErrorNcpType => Ok(Self::Error(Error::NcpType)),
            Values::AshErrorResetMethod => Ok(Self::Error(Error::ResetMethod)),
            Values::AshErrorXOnXOff => Ok(Self::Error(Error::XOnXOff)),
            Values::AshInProgress => Ok(Self::Misc(Misc::InProgress)),
            Values::AshStarted => Ok(Self::Misc(Misc::Started)),
            Values::AshConnected => Ok(Self::Misc(Misc::Connected)),
            Values::AshDisconnected => Ok(Self::Misc(Misc::Disconnected)),
            Values::AshAckTimeout => Ok(Self::Misc(Misc::AckTimeout)),
            Values::AshCancelled => Ok(Self::Misc(Misc::Cancelled)),
            Values::AshOutOfSequence => Ok(Self::Misc(Misc::OutOfSequence)),
            Values::AshBadCrc => Ok(Self::Misc(Misc::BadCrc)),
            Values::AshCommError => Ok(Self::Misc(Misc::CommError)),
            Values::AshBadAckNum => Ok(Self::Misc(Misc::BadAckNum)),
            Values::AshTooShort => Ok(Self::Misc(Misc::TooShort)),
            Values::AshTooLong => Ok(Self::Misc(Misc::TooLong)),
            Values::AshBadControl => Ok(Self::Misc(Misc::BadControl)),
            Values::AshBadLength => Ok(Self::Misc(Misc::BadLength)),
            Values::AshAckReceived => Ok(Self::Misc(Misc::AckReceived)),
            Values::AshAckSent => Ok(Self::Misc(Misc::AckSent)),
            Values::AshNakReceived => Ok(Self::Misc(Misc::NakReceived)),
            Values::AshNakSent => Ok(Self::Misc(Misc::NakSent)),
            Values::AshRstReceived => Ok(Self::Misc(Misc::RstReceived)),
            Values::AshRstSent => Ok(Self::Misc(Misc::RstSent)),
            Values::AshStatus => Ok(Self::Misc(Misc::Status)),
            Values::AshTx => Ok(Self::Misc(Misc::Tx)),
            Values::AshRx => Ok(Self::Misc(Misc::Rx)),
            value => Err(value),
        }
    }
}

impl From<Ash> for u8 {
    fn from(ash: Ash) -> Self {
        match ash {
            Ash::Error(error) => error.into(),
            Ash::Misc(misc) => misc.into(),
        }
    }
}

impl FromPrimitive for Ash {
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
