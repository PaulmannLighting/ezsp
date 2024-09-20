use crate::ezsp::status::values::Values;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Error {
    /// Fatal error detected by NCP.
    NcpFatalError,
    /// Incompatible ASH version.
    Version,
    /// Exceeded max ACK timeouts.
    Timeouts,
    /// Timed out waiting for RSTACK.
    ResetFail,
    /// Unexpected ncp reset.
    NcpReset,
    NcpType,
    ResetMethod,
    XOnXOff,
}

impl From<Error> for Values {
    fn from(error: Error) -> Self {
        match error {
            Error::NcpFatalError => Values::AshNcpFatalError,
            Error::Version => Values::AshErrorVersion,
            Error::Timeouts => Values::AshErrorTimeouts,
            Error::ResetFail => Values::AshErrorResetFail,
            Error::NcpReset => Values::AshErrorNcpReset,
            Error::NcpType => Values::AshErrorNcpType,
            Error::ResetMethod => Values::AshErrorResetMethod,
            Error::XOnXOff => Values::AshErrorXOnXOff,
        }
    }
}

impl TryFrom<Values> for Error {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::AshNcpFatalError => Ok(Self::NcpFatalError),
            Values::AshErrorVersion => Ok(Self::Version),
            Values::AshErrorTimeouts => Ok(Self::Timeouts),
            Values::AshErrorResetFail => Ok(Self::ResetFail),
            Values::AshErrorNcpReset => Ok(Self::NcpReset),
            Values::AshErrorNcpType => Ok(Self::NcpType),
            Values::AshErrorResetMethod => Ok(Self::ResetMethod),
            Values::AshErrorXOnXOff => Ok(Self::XOnXOff),
            value => Err(value),
        }
    }
}

impl From<Error> for u8 {
    fn from(error: Error) -> Self {
        Values::from(error).into()
    }
}

impl FromPrimitive for Error {
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
