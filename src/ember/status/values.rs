use num_derive::FromPrimitive;
use std::fmt::{Display, LowerHex, UpperHex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Values {
    Success = 0x00,
    ErrFatal = 0x01,
    BadArgument = 0x02,
    EepromMfgStackVersionMismatch = 0x04,
    EepromMfgVersionMismatch = 0x06,
    EepromStackVersionMismatch = 0x07,
    NoBuffers = 0x18,
    SerialInvalidBaudRate = 0x20,
    SerialInvalidPort = 0x21,
    SerialTxOverflow = 0x22,
    SerialRxOverflow = 0x23,
    SerialRxFrameError = 0x24,
    SerialRxParityError = 0x025,
    SerialRxEmpty = 0x26,
    SerialRxOverrunError = 0x27,
    MacNoData = 0x31,
    MacJoinedNetwork = 0x32,
    MacBadScanDuration = 0x33,
    MacIncorrectScanType = 0x34,
    MacInvalidChannelMask = 0x35,
    MacCommandTransmitFailure = 0x36,
    MacTransmitQueueFull = 0x39,
    MacUnknownHeaderType = 0x3A,
    MacScanning = 0x3D,
    MacNoAckReceived = 0x40,
    MacIndirectTimeout = 0x42,
    SimEepromErasePageGreen = 0x43,
    SimEepromErasePageRed = 0x44,
    SimEepromFull = 0x45,
    SimEepromInit1Failed = 0x48,
    SimEepromInit2Failed = 0x49,
    SimEepromInit3Failed = 0x4A,
    ErrFlashWriteInhibited = 0x46,
    ErrFlashVerifyFailed = 0x47,
    ErrFlashProgFail = 0x4B,
    ErrFlashEraseFail = 0x4C,
    ErrBootloaderTrapTableBad = 0x58,
    ErrBootloaderTrapUnknown = 0x59,
    ErrBootloaderNoImage = 0x5A,
    DeliveryFailed = 0x66,
    BindingIndexOutOfRange = 0x69,
    AddressTableIndexOutOfRange = 0x6A,
    InvalidBindingIndex = 0x6C,
    InvalidCall = 0x70,
    CostNotKnown = 0x71,
    MaxMessageLimitReached = 0x72,
    MessageTooLong = 0x74,
    BindingIsActive = 0x75,
    AddressTableEntryIsActive = 0x76,
    AdcConversionDone = 0x80,
    AdcConversionBusy = 0x81,
    AdcConversionDeferred = 0x82,
    AdcNoConversionPending = 0x84,
    SleepInterrupted = 0x85,
    PhyTxUnderflow = 0x88,
    PhyTxIncomplete = 0x89,
    PhyInvalidChannel = 0x8A,
    PhyInvalidPower = 0x8B,
    PhyTxBusy = 0x8C,
    PhyTxCcaFail = 0x8D,
    PhyOscillatorCheckFailed = 0x8E,
    PhyAckReceived = 0x8F,
    NetworkUp = 0x90,
    NetworkDown = 0x91,
    NotJoined = 0x93,
    JoinFailed = 0x94,
    InvalidSecurityLevel = 0x95,
    MoveFailed = 0x96,
    CannotJoinAsRouter = 0x98,
    NodeIdChanged = 0x99,
    PanIdChanged = 0x9A,
    NetworkOpened = 0x9C,
    NetworkClosed = 0x9D,
    NoBeacons = 0xAB,
    ReceivedKeyInTheClear = 0xAC,
    NoNetworkKeyReceived = 0xAD,
    NoLinkKeyReceived = 0xAE,
    PreconfiguredKeyRequired = 0xAF,
    KeyInvalid = 0xB2,
    NetworkBusy = 0xA1,
    InvalidEndpoint = 0xA3,
    BindingHasChanged = 0xA4,
    InsufficientRandomData = 0xA5,
    ApsEncryptionError = 0xA6,
    SecurityStateNotSet = 0xA8,
    SourceRouteFailure = 0xA9,
    ManyToOneRouteFailure = 0xAA,
    StackAndHardwareMismatch = 0xB0,
    IndexOutOfRange = 0xB1,
    KeyTableInvalidAddress = 0xB3,
    TableFull = 0xB4,
    LibraryNotPresent = 0xB5,
    TableEntryErased = 0xB6,
    SecurityConfigurationInvalid = 0xB7,
    TooSoonForSwitchKey = 0xB8,
    SignatureVerifyFailure = 0xB9,
    OperationInProgress = 0xBA,
    KeyNotAuthorized = 0xBB,
    SecurityDataInvalid = 0xBD,
    ApplicationError0 = 0xF0,
    ApplicationError1 = 0xF1,
    ApplicationError2 = 0xF2,
    ApplicationError3 = 0xF3,
    ApplicationError4 = 0xF4,
    ApplicationError5 = 0xF5,
    ApplicationError6 = 0xF6,
    ApplicationError7 = 0xF7,
    ApplicationError8 = 0xF8,
    ApplicationError9 = 0xF9,
    ApplicationError10 = 0xFA,
    ApplicationError11 = 0xFB,
    ApplicationError12 = 0xFC,
    ApplicationError13 = 0xFD,
    ApplicationError14 = 0xFE,
    ApplicationError15 = 0xFF,
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ember{self:?}")
    }
}

impl From<Values> for u8 {
    fn from(value: Values) -> Self {
        value as Self
    }
}

impl LowerHex for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", *self as u8)
    }
}

impl UpperHex for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04X}", *self as u8)
    }
}
