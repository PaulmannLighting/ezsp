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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
            Values::AshNcpFatalError => Self::Ash(Ash::Error(ash::Error::NcpFatal)),
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
            Status::Success => Self::Success,
            Status::SpiErr(spi_err) => match spi_err {
                SpiErr::Fatal => Self::SpiErrFatal,
                SpiErr::NcpReset => Self::SpiErrNcpReset,
                SpiErr::OversizedEzspFrame => Self::SpiErrOversizedEzspFrame,
                SpiErr::AbortedTransaction => Self::SpiErrAbortedTransaction,
                SpiErr::MissingFrameTerminator => Self::SpiErrMissingFrameTerminator,
                SpiErr::WaitSectionTimeout => Self::SpiErrWaitSectionTimeout,
                SpiErr::NoFrameTerminator => Self::SpiErrNoFrameTerminator,
                SpiErr::EzspCommandOversized => Self::SpiErrEzspCommandOversized,
                SpiErr::EzspResponseOversized => Self::SpiErrEzspResponseOversized,
                SpiErr::WaitingForResponse => Self::SpiWaitingForResponse,
                SpiErr::HandshakeTimeout => Self::SpiErrHandshakeTimeout,
                SpiErr::StartupTimeout => Self::SpiErrStartupTimeout,
                SpiErr::StartupFail => Self::SpiErrStartupFail,
                SpiErr::UnsupportedSpiCommand => Self::SpiErrUnsupportedSpiCommand,
            },
            Status::HostFatalError => Self::HostFatalError,
            Status::DataFrameTooLong => Self::DataFrameTooLong,
            Status::DataFrameTooShort => Self::DataFrameTooShort,
            Status::NoTxSpace => Self::NoTxSpace,
            Status::NoRxSpace => Self::NoRxSpace,
            Status::NoRxData => Self::NoRxData,
            Status::NotConnected => Self::NotConnected,
            Status::Error(error) => match error {
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
            },
            Status::Ash(ash) => match ash {
                Ash::Misc(misc) => match misc {
                    ash::Misc::InProgress => Self::AshInProgress,
                    ash::Misc::Started => Self::AshStarted,
                    ash::Misc::Connected => Self::AshConnected,
                    ash::Misc::Disconnected => Self::AshDisconnected,
                    ash::Misc::AckTimeout => Self::AshAckTimeout,
                    ash::Misc::Cancelled => Self::AshCancelled,
                    ash::Misc::OutOfSequence => Self::AshOutOfSequence,
                    ash::Misc::BadCrc => Self::AshBadCrc,
                    ash::Misc::CommError => Self::AshCommError,
                    ash::Misc::BadAckNum => Self::AshBadAckNum,
                    ash::Misc::TooShort => Self::AshTooShort,
                    ash::Misc::TooLong => Self::AshTooLong,
                    ash::Misc::BadControl => Self::AshBadControl,
                    ash::Misc::BadLength => Self::AshBadLength,
                    ash::Misc::AckReceived => Self::AshAckReceived,
                    ash::Misc::AckSent => Self::AshAckSent,
                    ash::Misc::NakReceived => Self::AshNakReceived,
                    ash::Misc::NakSent => Self::AshNakSent,
                    ash::Misc::RstReceived => Self::AshRstReceived,
                    ash::Misc::RstSent => Self::AshRstSent,
                    ash::Misc::Status => Self::AshStatus,
                    ash::Misc::Tx => Self::AshTx,
                    ash::Misc::Rx => Self::AshRx,
                },
                Ash::Error(error) => match error {
                    ash::Error::NcpFatal => Self::AshNcpFatalError,
                    ash::Error::Version => Self::AshErrorVersion,
                    ash::Error::Timeouts => Self::AshErrorTimeouts,
                    ash::Error::ResetFail => Self::AshErrorResetFail,
                    ash::Error::NcpReset => Self::AshErrorNcpReset,
                    ash::Error::NcpType => Self::AshErrorNcpType,
                    ash::Error::ResetMethod => Self::AshErrorResetMethod,
                    ash::Error::XOnXOff => Self::AshErrorXOnXOff,
                },
            },
            Status::CpcErrorInit => Self::CpcErrorInit,
            Status::NoError => Self::NoError,
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
