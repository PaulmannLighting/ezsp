use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Mac {
    NoData = 0x31,
    JoinedNetwork = 0x32,
    BadScanDuration = 0x33,
    IncorrectScanType = 0x34,
    InvalidChannelMask = 0x35,
    CommandTransmitFailure = 0x36,
    TransmitQueueFull = 0x39,
    UnknownHeaderType = 0x3A,
    Scanning = 0x3D,
    NoAckReceived = 0x40,
    IndirectTimeout = 0x42,
}
