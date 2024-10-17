use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, LowerHex, UpperHex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Values {
    Success = 0x00,
    SpiErrFatal = 0x10,
    SpiErrNcpReset = 0x11,
    SpiErrOversizedEzspFrame = 0x12,
    SpiErrAbortedTransaction = 0x13,
    SpiErrMissingFrameTerminator,
    SpiErrWaitSectionTimeout = 0x15,
    SpiErrNoFrameTerminator = 0x16,
    SpiErrEzspCommandOversized = 0x17,
    SpiErrEzspResponseOversized = 0x18,
    SpiWaitingForResponse = 0x19,
    SpiErrHandshakeTimeout = 0x1A,
    SpiErrStartupTimeout = 0x1B,
    SpiErrStartupFail = 0x1C,
    SpiErrUnsupportedSpiCommand = 0x1D,
    AshInProgress = 0x20,
    HostFatalError = 0x21,
    AshNcpFatalError = 0x22,
    DataFrameTooLong = 0x23,
    DataFrameTooShort = 0x24,
    NoTxSpace = 0x25,
    NoRxSpace = 0x26,
    NoRxData = 0x27,
    NotConnected = 0x28,
    ErrorVersionNotSet = 0x30,
    ErrorInvalidFrameId = 0x31,
    ErrorWrongDirection = 0x32,
    ErrorTruncated = 0x33,
    ErrorOverflow = 0x34,
    ErrorOutOfMemory = 0x35,
    ErrorInvalidValue = 0x36,
    ErrorInvalidId = 0x37,
    ErrorInvalidCall = 0x38,
    ErrorNoResponse = 0x39,
    ErrorCommandTooLong = 0x40,
    ErrorQueueFull = 0x41,
    ErrorCommandFiltered = 0x42,
    ErrorSecurityKeyAlreadySet = 0x43,
    ErrorSecurityTypeInvalid = 0x44,
    ErrorSecurityParametersInvalid = 0x45,
    ErrorSecurityParametersAlreadySet = 0x46,
    ErrorSecurityKeyNotSet = 0x47,
    ErrorSecurityParametersNotSet = 0x48,
    ErrorUnsupportedControl = 0x49,
    ErrorUnsecureFrame = 0x4A,
    AshErrorVersion = 0x50,
    AshErrorTimeouts = 0x51,
    AshErrorResetFail = 0x52,
    AshErrorNcpReset = 0x53,
    ErrorSerialInit = 0x54,
    AshErrorNcpType = 0x55,
    AshErrorResetMethod = 0x56,
    AshErrorXOnXOff = 0x57,
    AshStarted = 0x70,
    AshConnected = 0x71,
    AshDisconnected = 0x72,
    AshAckTimeout = 0x73,
    AshCancelled = 0x74,
    AshOutOfSequence = 0x75,
    AshBadCrc = 0x76,
    AshCommError = 0x77,
    AshBadAckNum = 0x78,
    AshTooShort = 0x79,
    AshTooLong = 0x7A,
    AshBadControl = 0x7B,
    AshBadLength = 0x7C,
    AshAckReceived = 0x7D,
    AshAckSent = 0x7E,
    AshNakReceived = 0x7F,
    AshNakSent = 0x80,
    AshRstReceived = 0x81,
    AshRstSent = 0x82,
    AshStatus = 0x83,
    AshTx = 0x84,
    AshRx = 0x85,
    CpcErrorInit = 0x86,
    NoError = 0xFF,
}

impl Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ezsp{self:?}")
    }
}

impl From<Values> for u8 {
    fn from(values: Values) -> Self {
        values as Self
    }
}

impl LowerHex for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", *self as u8)
    }
}

impl UpperHex for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", *self as u8)
    }
}
