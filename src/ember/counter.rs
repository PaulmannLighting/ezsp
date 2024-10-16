//! Ember counter type.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember counter type
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Type {
    /// The MAC received a broadcast.
    MacRxBroadcast = 0,
    /// The MAC transmitted a broadcast.
    MacTxBroadcast = 1,
    /// The MAC received a unicast.
    MacRxUnicast = 2,
    /// The MAC successfully transmitted a unicast.
    MacTxUnicastSuccess = 3,
    /// The MAC retried a unicast.
    MacTxUnicastRetry = 4,
    /// The MAC unsuccessfully transmitted a unicast.
    MacTxUnicastFailed = 5,
    /// The APS layer received a data broadcast.
    ApsDataRxBroadcast = 6,
    /// The APS layer transmitted a data broadcast.
    ApsDataTxBroadcast = 7,
    /// The APS layer received a data unicast.
    ApsDataRxUnicast = 8,
    /// The APS layer successfully transmitted a data unicast.
    ApsDataTxUnicastSuccess = 9,
    /// The APS layer retried a data unicast.
    ApsDataTxUnicastRetry = 10,
    /// The APS layer unsuccessfully transmitted a data unicast.
    ApsDataTxUnicastFailed = 11,
    /// The network layer successfully submitted a new  route discovery to the MAC.
    RouteDiscoveryInitiated = 12,
    /// An entry was added to the neighbor table.
    NeighborAdded = 13,
    /// An entry was removed from the neighbor table.
    NeighborRemoved = 14,
    /// A neighbor table entry became stale because it had not been heard from.
    NeighborStale = 15,
    /// A node joined or rejoined to the network via this node.
    JoinIndication = 16,
    /// An entry was removed from the child table.
    ChildRemoved = 17,
    /// EZSP-UART only. An overflow error occurred in the UART.
    AshOverflowError = 18,
    /// EZSP-UART only. A framing error occurred in the UART.
    AshFramingError = 19,
    /// EZSP-UART only. An overrun error occurred in the UART.
    AshOverrunError = 20,
    /// A message was dropped at the network layer because the NWK frame counter was not higher
    /// than the last message seen from that source.
    NwkFrameCounterFailure = 21,
    /// A message was dropped at the APS layer because the APS frame counter was not higher
    /// than the last message seen from that source.
    ApsFrameCounterFailure = 22,
    /// Utility counter for general debugging use.
    Utility = 23,
    /// A message was dropped at the APS layer because it had APS encryption but the key
    /// associated with the sender has not been authenticated,
    /// and thus the key is not authorized for use in APS data messages.
    ApsLinkKeyNotAuthorized = 24,
    /// An NWK-encrypted message was received but dropped because decryption failed.
    NwkDecryptionFailure = 25,
    /// An APS encrypted message was received but dropped because decryption failed.
    ApsDecryptionFailure = 26,
    /// The number of times we failed to allocate a set of linked packet buffers.
    ///
    /// This doesn't necessarily mean that the packet buffer count was 0 at the time,
    /// but that the number requested was greater than the number free.
    AllocatePacketBufferFailure = 27,
    /// The number of relayed unicast packets.
    RelayedUnicast = 28,
    /// The number of times we dropped a packet due to reaching the preset PHY to MAC queue limit
    /// (sli_802154mac_max_phy_to_mac_queue_length).
    PhyToMacQueueLimitReached = 29,
    /// The number of times we dropped a packet due to the packet-validate library checking a
    /// packet and rejecting it due to length or other formatting problems.
    PacketValidateLibraryDroppedCount = 30,
    /// The number of times the NWK retry queue is full and a new message failed to be added.
    TypeNwkRetryOverflow = 31,
    /// The number of times the PHY layer was unable to transmit due to a failed CCA.
    PhyCcaFailCount = 32,
    /// The number of times an NWK broadcast was dropped because the broadcast table was full.
    BroadcastTableFull = 33,
    /// The number of low priority packet traffic arbitration requests.
    PtaLoPriRequested = 34,
    /// The number of high priority packet traffic arbitration requests.
    PtaHiPriRequested = 35,
    /// The number of low priority packet traffic arbitration requests denied.
    PtaLoPriDenied = 36,
    /// The number of high priority packet traffic arbitration requests denied.
    PtaHiPriDenied = 37,
    /// The number of aborted low priority packet traffic arbitration transmissions.
    PtaLoPriTxAborted = 38,
    /// The number of aborted high priority packet traffic arbitration transmissions.
    PtaHiPriTxAborted = 39,
    /// A placeholder giving the number of Ember counter types.
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
