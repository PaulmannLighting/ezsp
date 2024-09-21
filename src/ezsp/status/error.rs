use super::values::Values;
use num_traits::FromPrimitive;

/// EZSP errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The NCP received a command before the EZSP version had been set.
    VersionNotSet,
    /// The NCP received a command containing an unsupported frame ID.
    InvalidFrameId,
    /// The direction flag in the frame control field was incorrect.
    WrongDirection,
    /// The truncated flag in the frame control field was set,
    /// indicating there was not enough memory available to complete the response
    /// or that the response would have exceeded the maximum EZSP frame length.
    Truncated,
    /// The overflow flag in the frame control field was set,
    /// indicating one or more callbacks occurred since the previous response
    /// and there was not enough memory available to report them to the Host.
    Overflow,
    /// Insufficient memory was available.
    OutOfMemory,
    /// The value was out of bounds.
    InvalidValue,
    /// The configuration id was not recognized.
    InvalidId,
    /// Configuration values can no longer be modified.
    InvalidCall,
    /// The NCP failed to respond to a command.
    NoResponse,
    /// The length of the command exceeded the maximum EZSP frame length.
    CommandTooLong,
    /// The UART receive queue was full causing a callback response to be dropped.
    QueueFull,
    /// The command has been filtered out by NCP.
    CommandFiltered,
    /// EZSP Security Key is already set.
    SecurityKeyAlreadySet,
    /// EZSP Security Type is invalid.
    SecurityTypeInvalid,
    /// EZSP Security Parameters are invalid.
    SecurityParametersInvalid,
    /// EZSP Security Parameters are already set.
    SecurityParametersAlreadySet,
    /// EZSP Security Key is not set.
    SecurityKeyNotSet,
    /// EZSP Security Parameters are not set.
    SecurityParametersNotSet,
    /// Received frame with unsupported control byte.
    UnsupportedControl,
    /// Received frame is unsecure, when security is established.
    UnsecureFrame,
    /// Serial port initialization failed.
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
            _ => Err(value),
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
