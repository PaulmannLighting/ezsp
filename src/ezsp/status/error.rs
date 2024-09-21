use crate::ezsp::status::values::Values;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    VersionNotSet,
    InvalidFrameId,
    WrongDirection,
    Truncated,
    Overflow,
    OutOfMemory,
    InvalidValue,
    InvalidId,
    InvalidCall,
    NoResponse,
    CommandTooLong,
    QueueFull,
    CommandFiltered,
    SecurityKeyAlreadySet,
    SecurityTypeInvalid,
    SecurityParametersInvalid,
    SecurityParametersAlreadySet,
    SecurityKeyNotSet,
    SecurityParametersNotSet,
    UnsupportedControl,
    UnsecureFrame,
    SerialInit,
}

impl From<Error> for Values {
    fn from(error: Error) -> Self {
        match error {
            Error::VersionNotSet => Self::ErrorVersionNotSet,
            Error::InvalidFrameId => Self::ErrorInvalidFrameId,
            Error::WrongDirection => Self::ErrorWrongDirection,
            Error::Truncated => Self::ErrorTruncated,
            Error::Overflow => Self::ErrorOverflow,
            Error::OutOfMemory => Self::ErrorOutOfMemory,
            Error::InvalidValue => Self::ErrorInvalidValue,
            Error::InvalidId => Self::ErrorInvalidId,
            Error::InvalidCall => Self::ErrorInvalidCall,
            Error::NoResponse => Self::ErrorNoResponse,
            Error::CommandTooLong => Self::ErrorCommandTooLong,
            Error::QueueFull => Self::ErrorQueueFull,
            Error::CommandFiltered => Self::ErrorCommandFiltered,
            Error::SecurityKeyAlreadySet => Self::ErrorSecurityKeyAlreadySet,
            Error::SecurityTypeInvalid => Self::ErrorSecurityTypeInvalid,
            Error::SecurityParametersInvalid => Self::ErrorSecurityParametersInvalid,
            Error::SecurityParametersAlreadySet => Self::ErrorSecurityParametersAlreadySet,
            Error::SecurityKeyNotSet => Self::ErrorSecurityKeyNotSet,
            Error::SecurityParametersNotSet => Self::ErrorSecurityParametersNotSet,
            Error::UnsupportedControl => Self::ErrorUnsupportedControl,
            Error::UnsecureFrame => Self::ErrorUnsecureFrame,
            Error::SerialInit => Self::ErrorSerialInit,
        }
    }
}

impl TryFrom<Values> for Error {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::ErrorVersionNotSet => Ok(Self::VersionNotSet),
            Values::ErrorInvalidFrameId => Ok(Self::InvalidFrameId),
            Values::ErrorWrongDirection => Ok(Self::WrongDirection),
            Values::ErrorTruncated => Ok(Self::Truncated),
            Values::ErrorOverflow => Ok(Self::Overflow),
            Values::ErrorOutOfMemory => Ok(Self::OutOfMemory),
            Values::ErrorInvalidValue => Ok(Self::InvalidValue),
            Values::ErrorInvalidId => Ok(Self::InvalidId),
            Values::ErrorInvalidCall => Ok(Self::InvalidCall),
            Values::ErrorNoResponse => Ok(Self::NoResponse),
            Values::ErrorCommandTooLong => Ok(Self::CommandTooLong),
            Values::ErrorQueueFull => Ok(Self::QueueFull),
            Values::ErrorCommandFiltered => Ok(Self::CommandFiltered),
            Values::ErrorSecurityKeyAlreadySet => Ok(Self::SecurityKeyAlreadySet),
            Values::ErrorSecurityTypeInvalid => Ok(Self::SecurityTypeInvalid),
            Values::ErrorSecurityParametersInvalid => Ok(Self::SecurityParametersInvalid),
            Values::ErrorSecurityParametersAlreadySet => Ok(Self::SecurityParametersAlreadySet),
            Values::ErrorSecurityKeyNotSet => Ok(Self::SecurityKeyNotSet),
            Values::ErrorSecurityParametersNotSet => Ok(Self::SecurityParametersNotSet),
            Values::ErrorUnsupportedControl => Ok(Self::UnsupportedControl),
            Values::ErrorUnsecureFrame => Ok(Self::UnsecureFrame),
            Values::ErrorSerialInit => Ok(Self::SerialInit),
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
