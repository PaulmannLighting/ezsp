use num_traits::FromPrimitive;

use crate::Resolve;
pub use ash::Ash;
pub use error::Error;
pub use spi_err::SpiErr;
use values::Values;

mod ash;
mod error;
mod spi_err;
mod values;

/// Status values used by EZSP.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Status {
    /// Success.
    Success,
    /// SPI-related status.
    SpiErr(SpiErr),
    /// Fatal error detected by host.
    HostFatalError,
    /// Tried to send DATA frame too long.
    DataFrameTooLong,
    /// Tried to send DATA frame too short.
    DataFrameTooShort,
    /// No space for tx'ed DATA frame.
    NoTxSpace,
    /// No space for rec'd DATA frame.
    NoRxSpace,
    /// No receive data available.
    NoRxData,
    /// Not in Connected state.
    NotConnected,
    /// Errors status.
    Error(Error),
    /// ASH-related status.
    Ash(Ash),
    /// Failed to connect to CPC daemon or failed to open CPC endpoint.
    CpcErrorInit,
    /// No reset or error.
    NoError,
}

impl Status {
    /// Checks the status for success and returns `Ok(value)` in that case.
    ///
    /// # Errors
    /// Returns `Err(self)` if the `Status` is not [`Status::Success`],
    pub fn map<T>(self, value: T) -> Result<T, Self> {
        if self == Self::Success {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// Checks the status for success and returns `Ok(())` in that case.
    ///
    /// # Errors
    /// Returns `Err(self)` if the `Status` is not [`Status::Success`],
    pub fn ok(self) -> Result<(), Self> {
        self.map(())
    }
}

impl From<Values> for Status {
    fn from(value: Values) -> Self {
        match value {
            Values::Success => Self::Success,
            Values::SpiErrFatal => Self::SpiErr(SpiErr::Fatal),
            Values::SpiErrNcpReset => Self::SpiErr(SpiErr::NcpReset),
            Values::SpiErrOversizedEzspFrame => Self::SpiErr(SpiErr::OversizedEzspFrame),
            Values::SpiErrAbortedTransaction => Self::SpiErr(SpiErr::AbortedTransaction),
            Values::SpiErrMissingFrameTerminator => Self::SpiErr(SpiErr::MissingFrameTerminator),
            Values::SpiErrWaitSectionTimeout => Self::SpiErr(SpiErr::WaitSectionTimeout),
            Values::SpiErrNoFrameTerminator => Self::SpiErr(SpiErr::NoFrameTerminator),
            Values::SpiErrEzspCommandOversized => Self::SpiErr(SpiErr::EzspCommandOversized),
            Values::SpiErrEzspResponseOversized => Self::SpiErr(SpiErr::EzspResponseOversized),
            Values::SpiWaitingForResponse => Self::SpiErr(SpiErr::WaitingForResponse),
            Values::SpiErrHandshakeTimeout => Self::SpiErr(SpiErr::HandshakeTimeout),
            Values::SpiErrStartupTimeout => Self::SpiErr(SpiErr::StartupTimeout),
            Values::SpiErrStartupFail => Self::SpiErr(SpiErr::StartupFail),
            Values::SpiErrUnsupportedSpiCommand => Self::SpiErr(SpiErr::UnsupportedSpiCommand),
            Values::AshInProgress => Self::Ash(Ash::Misc(ash::Misc::InProgress)),
            Values::HostFatalError => Self::HostFatalError,
            Values::AshNcpFatalError => Self::Ash(Ash::Error(ash::Error::NcpFatalError)),
            Values::DataFrameTooLong => Self::DataFrameTooLong,
            Values::DataFrameTooShort => Self::DataFrameTooShort,
            Values::NoTxSpace => Self::NoTxSpace,
            Values::NoRxSpace => Self::NoRxSpace,
            Values::NoRxData => Self::NoRxData,
            Values::NotConnected => Self::NotConnected,
            Values::ErrorVersionNotSet => Self::Error(Error::VersionNotSet),
            Values::ErrorInvalidFrameId => Self::Error(Error::InvalidFrameId),
            Values::ErrorWrongDirection => Self::Error(Error::WrongDirection),
            Values::ErrorTruncated => Self::Error(Error::Truncated),
            Values::ErrorOverflow => Self::Error(Error::Overflow),
            Values::ErrorOutOfMemory => Self::Error(Error::OutOfMemory),
            Values::ErrorInvalidValue => Self::Error(Error::InvalidValue),
            Values::ErrorInvalidId => Self::Error(Error::InvalidId),
            Values::ErrorInvalidCall => Self::Error(Error::InvalidCall),
            Values::ErrorNoResponse => Self::Error(Error::NoResponse),
            Values::ErrorCommandTooLong => Self::Error(Error::CommandTooLong),
            Values::ErrorQueueFull => Self::Error(Error::QueueFull),
            Values::ErrorCommandFiltered => Self::Error(Error::CommandFiltered),
            Values::ErrorSecurityKeyAlreadySet => Self::Error(Error::SecurityKeyAlreadySet),
            Values::ErrorSecurityTypeInvalid => Self::Error(Error::SecurityTypeInvalid),
            Values::ErrorSecurityParametersInvalid => Self::Error(Error::SecurityParametersInvalid),
            Values::ErrorSecurityParametersAlreadySet => {
                Self::Error(Error::SecurityParametersAlreadySet)
            }
            Values::ErrorSecurityKeyNotSet => Self::Error(Error::SecurityKeyNotSet),
            Values::ErrorSecurityParametersNotSet => Self::Error(Error::SecurityParametersNotSet),
            Values::ErrorUnsupportedControl => Self::Error(Error::UnsupportedControl),
            Values::ErrorUnsecureFrame => Self::Error(Error::UnsecureFrame),
            Values::AshErrorVersion => Self::Ash(Ash::Error(ash::Error::Version)),
            Values::AshErrorTimeouts => Self::Ash(Ash::Error(ash::Error::Timeouts)),
            Values::AshErrorResetFail => Self::Ash(Ash::Error(ash::Error::ResetFail)),
            Values::AshErrorNcpReset => Self::Ash(Ash::Error(ash::Error::NcpReset)),
            Values::ErrorSerialInit => Self::Error(Error::SerialInit),
            Values::AshErrorNcpType => Self::Ash(Ash::Error(ash::Error::NcpType)),
            Values::AshErrorResetMethod => Self::Ash(Ash::Error(ash::Error::ResetMethod)),
            Values::AshErrorXOnXOff => Self::Ash(Ash::Error(ash::Error::XOnXOff)),
            Values::AshStarted => Self::Ash(Ash::Misc(ash::Misc::Started)),
            Values::AshConnected => Self::Ash(Ash::Misc(ash::Misc::Connected)),
            Values::AshDisconnected => Self::Ash(Ash::Misc(ash::Misc::Disconnected)),
            Values::AshAckTimeout => Self::Ash(Ash::Misc(ash::Misc::AckTimeout)),
            Values::AshCancelled => Self::Ash(Ash::Misc(ash::Misc::Cancelled)),
            Values::AshOutOfSequence => Self::Ash(Ash::Misc(ash::Misc::OutOfSequence)),
            Values::AshBadCrc => Self::Ash(Ash::Misc(ash::Misc::BadCrc)),
            Values::AshCommError => Self::Ash(Ash::Misc(ash::Misc::CommError)),
            Values::AshBadAckNum => Self::Ash(Ash::Misc(ash::Misc::BadAckNum)),
            Values::AshTooShort => Self::Ash(Ash::Misc(ash::Misc::TooShort)),
            Values::AshTooLong => Self::Ash(Ash::Misc(ash::Misc::TooLong)),
            Values::AshBadControl => Self::Ash(Ash::Misc(ash::Misc::BadControl)),
            Values::AshBadLength => Self::Ash(Ash::Misc(ash::Misc::BadLength)),
            Values::AshAckReceived => Self::Ash(Ash::Misc(ash::Misc::AckReceived)),
            Values::AshAckSent => Self::Ash(Ash::Misc(ash::Misc::AckSent)),
            Values::AshNakReceived => Self::Ash(Ash::Misc(ash::Misc::NakReceived)),
            Values::AshNakSent => Self::Ash(Ash::Misc(ash::Misc::NakSent)),
            Values::AshRstReceived => Self::Ash(Ash::Misc(ash::Misc::RstReceived)),
            Values::AshRstSent => Self::Ash(Ash::Misc(ash::Misc::RstSent)),
            Values::AshStatus => Self::Ash(Ash::Misc(ash::Misc::Status)),
            Values::AshTx => Self::Ash(Ash::Misc(ash::Misc::Tx)),
            Values::AshRx => Self::Ash(Ash::Misc(ash::Misc::Rx)),
            Values::CpcErrorInit => Self::CpcErrorInit,
            Values::NoError => Self::NoError,
        }
    }
}

impl From<Status> for Values {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => Values::Success,
            Status::SpiErr(spi_err) => match spi_err {
                SpiErr::Fatal => Values::SpiErrFatal,
                SpiErr::NcpReset => Values::SpiErrNcpReset,
                SpiErr::OversizedEzspFrame => Values::SpiErrOversizedEzspFrame,
                SpiErr::AbortedTransaction => Values::SpiErrAbortedTransaction,
                SpiErr::MissingFrameTerminator => Values::SpiErrMissingFrameTerminator,
                SpiErr::WaitSectionTimeout => Values::SpiErrWaitSectionTimeout,
                SpiErr::NoFrameTerminator => Values::SpiErrNoFrameTerminator,
                SpiErr::EzspCommandOversized => Values::SpiErrEzspCommandOversized,
                SpiErr::EzspResponseOversized => Values::SpiErrEzspResponseOversized,
                SpiErr::WaitingForResponse => Values::SpiWaitingForResponse,
                SpiErr::HandshakeTimeout => Values::SpiErrHandshakeTimeout,
                SpiErr::StartupTimeout => Values::SpiErrStartupTimeout,
                SpiErr::StartupFail => Values::SpiErrStartupFail,
                SpiErr::UnsupportedSpiCommand => Values::SpiErrUnsupportedSpiCommand,
            },
            Status::HostFatalError => Values::HostFatalError,
            Status::DataFrameTooLong => Values::DataFrameTooLong,
            Status::DataFrameTooShort => Values::DataFrameTooShort,
            Status::NoTxSpace => Values::NoTxSpace,
            Status::NoRxSpace => Values::NoRxSpace,
            Status::NoRxData => Values::NoRxData,
            Status::NotConnected => Values::NotConnected,
            Status::Error(error) => match error {
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
            },
            Status::Ash(ash) => match ash {
                Ash::Misc(misc) => match misc {
                    ash::Misc::InProgress => Values::AshInProgress,
                    ash::Misc::Started => Values::AshStarted,
                    ash::Misc::Connected => Values::AshConnected,
                    ash::Misc::Disconnected => Values::AshDisconnected,
                    ash::Misc::AckTimeout => Values::AshAckTimeout,
                    ash::Misc::Cancelled => Values::AshCancelled,
                    ash::Misc::OutOfSequence => Values::AshOutOfSequence,
                    ash::Misc::BadCrc => Values::AshBadCrc,
                    ash::Misc::CommError => Values::AshCommError,
                    ash::Misc::BadAckNum => Values::AshBadAckNum,
                    ash::Misc::TooShort => Values::AshTooShort,
                    ash::Misc::TooLong => Values::AshTooLong,
                    ash::Misc::BadControl => Values::AshBadControl,
                    ash::Misc::BadLength => Values::AshBadLength,
                    ash::Misc::AckReceived => Values::AshAckReceived,
                    ash::Misc::AckSent => Values::AshAckSent,
                    ash::Misc::NakReceived => Values::AshNakReceived,
                    ash::Misc::NakSent => Values::AshNakSent,
                    ash::Misc::RstReceived => Values::AshRstReceived,
                    ash::Misc::RstSent => Values::AshRstSent,
                    ash::Misc::Status => Values::AshStatus,
                    ash::Misc::Tx => Values::AshTx,
                    ash::Misc::Rx => Values::AshRx,
                },
                Ash::Error(error) => match error {
                    ash::Error::NcpFatalError => Values::AshNcpFatalError,
                    ash::Error::Version => Values::AshErrorVersion,
                    ash::Error::Timeouts => Values::AshErrorTimeouts,
                    ash::Error::ResetFail => Values::AshErrorResetFail,
                    ash::Error::NcpReset => Values::AshErrorNcpReset,
                    ash::Error::NcpType => Values::AshErrorNcpType,
                    ash::Error::ResetMethod => Values::AshErrorResetMethod,
                    ash::Error::XOnXOff => Values::AshErrorXOnXOff,
                },
            },
            Status::CpcErrorInit => Values::CpcErrorInit,
            Status::NoError => Values::NoError,
        }
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        Values::from(status).into()
    }
}

impl FromPrimitive for Status {
    fn from_i64(n: i64) -> Option<Self> {
        Values::from_i64(n).map(Self::from)
    }

    fn from_u8(n: u8) -> Option<Self> {
        Values::from_u8(n).map(Self::from)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Values::from_u64(n).map(Self::from)
    }
}

impl Resolve for Result<Status, u8> {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        match self {
            Ok(status) => status.ok().map_err(crate::Error::Ezsp),
            Err(status) => Err(crate::Error::InvalidEzspStatus(status)),
        }
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
