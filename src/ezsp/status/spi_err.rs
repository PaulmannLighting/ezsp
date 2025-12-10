use num_traits::FromPrimitive;

use super::values::Values;

/// SPI-related errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpiErr {
    /// Fatal error.
    Fatal,
    /// The Response frame of the current defragmentation indicates the NCP has reset.
    NcpReset,
    /// The NCP is reporting that the Command frame of the current defragmentation
    /// is oversized (the length byte is too large).
    OversizedEzspFrame,
    /// The Response frame of the current defragmentation indicates the
    /// previous defragmentation was aborted (nSSEL deasserted too soon).
    AbortedTransaction,
    /// The Response frame of the current defragmentation indicates the
    /// frame terminator is missing from the Command frame.
    MissingFrameTerminator,
    /// The NCP has not provided a Response within the time limit defined by `WAIT_SECTION_TIMEOUT`.
    WaitSectionTimeout,
    /// The Response frame from the NCP is missing the frame terminator.
    NoFrameTerminator,
    /// The Host attempted to send an oversized Command (the length byte is too large)
    /// and the AVR's spi-protocol.c blocked the transmission.
    EzspCommandOversized,
    /// The NCP attempted to send an oversized Response (the length byte is too large)
    /// and the AVR's spi-protocol.c blocked the reception.
    EzspResponseOversized,
    /// The Host has sent the Command and is still waiting for the NCP to send a Response.
    WaitingForResponse,
    /// The NCP has not asserted `nHOST_INT` within the time limit
    /// defined by `WAKE_HANDSHAKE_TIMEOUT`.
    HandshakeTimeout,
    /// The NCP has not asserted `nHOST_INT` after an NCP reset
    /// within the time limit defined by `STARTUP_TIMEOUT`.
    StartupTimeout,
    /// The Host attempted to verify the SPI Protocol activity and version number,
    /// and the verification failed.
    StartupFail,
    /// The Host has sent a command with a SPI Byte that is
    /// unsupported by the current mode the NCP is operating in.
    UnsupportedSpiCommand,
}

impl From<SpiErr> for Values {
    fn from(spi_err: SpiErr) -> Self {
        match spi_err {
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
        }
    }
}

impl TryFrom<Values> for SpiErr {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::SpiErrFatal => Ok(Self::Fatal),
            Values::SpiErrNcpReset => Ok(Self::NcpReset),
            Values::SpiErrOversizedEzspFrame => Ok(Self::OversizedEzspFrame),
            Values::SpiErrAbortedTransaction => Ok(Self::AbortedTransaction),
            Values::SpiErrMissingFrameTerminator => Ok(Self::MissingFrameTerminator),
            Values::SpiErrWaitSectionTimeout => Ok(Self::WaitSectionTimeout),
            Values::SpiErrNoFrameTerminator => Ok(Self::NoFrameTerminator),
            Values::SpiErrEzspCommandOversized => Ok(Self::EzspCommandOversized),
            Values::SpiErrEzspResponseOversized => Ok(Self::EzspResponseOversized),
            Values::SpiWaitingForResponse => Ok(Self::WaitingForResponse),
            Values::SpiErrHandshakeTimeout => Ok(Self::HandshakeTimeout),
            Values::SpiErrStartupTimeout => Ok(Self::StartupTimeout),
            Values::SpiErrStartupFail => Ok(Self::StartupFail),
            Values::SpiErrUnsupportedSpiCommand => Ok(Self::UnsupportedSpiCommand),
            _ => Err(value),
        }
    }
}

impl From<SpiErr> for u8 {
    fn from(spi_err: SpiErr) -> Self {
        Values::from(spi_err).into()
    }
}

impl FromPrimitive for SpiErr {
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
