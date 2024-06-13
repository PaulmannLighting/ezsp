use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Type {
    MacRxBroadcast = 0,
    MacTxBroadcast = 1,
    MacRxUnicast = 2,
    MacTxUnicastSuccess = 3,
    MacTxUnicastRetry = 4,
    MacTxUnicastFailed = 5,
    ApsDataRxBroadcast = 6,
    ApsDataTxBroadcast = 7,
    ApsDataRxUnicast = 8,
    ApsDataTxUnicastSuccess = 9,
    ApsDataTxUnicastRetry = 10,
    ApsDataTxUnicastFailed = 11,
    RouteDiscoveryInitiated = 12,
    NeighborAdded = 13,
    NeighborRemoved = 14,
    NeighborStale = 15,
    JoinIndication = 16,
    ChildRemoved = 17,
    AshOverflowError = 18,
    AshFramingError = 19,
    AshOverrunError = 20,
    NwkFrameCounterFailure = 21,
    ApsFrameCounterFailure = 22,
    Utility = 23,
    ApsLinkKeyNotAuthorized = 24,
    NwkDecryptionFailure = 25,
    ApsDecryptionFailure = 26,
    AllocatePacketBufferFailure = 27,
    RelayedUnicast = 28,
    PhyToMacQueueLimitReached = 29,
    PacketValidateLibraryDroppedCount = 30,
    TypeNwkRetryOverflow = 31,
    PhyCcaFailCount = 32,
    BroadcastTableFull = 33,
    PtaLoPriRequested = 34,
    PtaHiPriRequested = 35,
    PtaLoPriDenied = 36,
    PtaHiPriDenied = 37,
    PtaLoPriTxAborted = 38,
    PtaHiPriTxAborted = 39,
    TypeCount = 40,
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ as Self
    }
}

impl TryFrom<u8> for Type {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
