use crate::ezsp::status::values::Values;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
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
