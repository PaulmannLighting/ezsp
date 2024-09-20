use crate::ezsp::status::values::Values;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
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
            Error::VersionNotSet => Values::ErrorVersionNotSet,
            Error::InvalidFrameId => Values::ErrorInvalidFrameId,
            Error::WrongDirection => Values::ErrorWrongDirection,
            Error::Truncated => Values::ErrorTruncated,
            Error::Overflow => Values::ErrorOverflow,
            Error::OutOfMemory => Values::ErrorOutOfMemory,
            Error::InvalidValue => Values::ErrorInvalidValue,
            Error::InvalidId => Values::ErrorInvalidId,
            Error::InvalidCall => Values::ErrorInvalidCall,
            Error::NoResponse => Values::ErrorNoResponse,
            Error::CommandTooLong => Values::ErrorCommandTooLong,
            Error::QueueFull => Values::ErrorQueueFull,
            Error::CommandFiltered => Values::ErrorCommandFiltered,
            Error::SecurityKeyAlreadySet => Values::ErrorSecurityKeyAlreadySet,
            Error::SecurityTypeInvalid => Values::ErrorSecurityTypeInvalid,
            Error::SecurityParametersInvalid => Values::ErrorSecurityParametersInvalid,
            Error::SecurityParametersAlreadySet => Values::ErrorSecurityParametersAlreadySet,
            Error::SecurityKeyNotSet => Values::ErrorSecurityKeyNotSet,
            Error::SecurityParametersNotSet => Values::ErrorSecurityParametersNotSet,
            Error::UnsupportedControl => Values::ErrorUnsupportedControl,
            Error::UnsecureFrame => Values::ErrorUnsecureFrame,
            Error::SerialInit => Values::ErrorSerialInit,
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
