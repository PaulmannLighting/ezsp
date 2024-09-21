use crate::ezsp::status::values::Values;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpiErr {
    Fatal,
    NcpReset,
    OversizedEzspFrame,
    AbortedTransaction,
    MissingFrameTerminator,
    WaitSectionTimeout,
    NoFrameTerminator,
    EzspCommandOversized,
    EzspResponseOversized,
    WaitingForResponse,
    HandshakeTimeout,
    StartupTimeout,
    StartupFail,
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
            value => Err(value),
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
