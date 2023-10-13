use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum SpiErr {
    Fatal = 0x10,
    NcpReset = 0x11,
    OversizedEzpFrame = 0x12,
    AbortedTransaction = 0x13,
    MissingFrameTerminator = 0x14,
    WaitSectionTimeout = 0x15,
    NoFrameTerminator = 0x16,
    EzspCommandOversized = 0x17,
    EzspResponseOversized = 0x18,
    WaitingForResponse = 0x19,
    HandshakeTimeout = 0x1A,
    StartupTimeout = 0x1B,
    StartupFail = 0x1C,
    UnsupportedSpiCommand = 0x1D,
}
