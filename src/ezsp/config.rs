use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Identifies a configuration value.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Id {
    /// The NCP no longer supports configuration of packet buffer count at runtime
    /// using this parameter.
    ///
    /// Packet buffers must be configured using the `EMBER_PACKET_BUFFER_COUNT` macro
    /// when building the NCP project.
    PacketBufferCount = 0x01,
    /// The maximum number of router neighbors the stack can keep track of.
    ///
    /// A neighbor is a node within radio range.
    NeighborTableSize = 0x02,
    /// The maximum number of APS retried messages the stack can be transmitting at any time.
    ApsUnicastMessageCount = 0x03,
    /// The maximum number of non-volatile bindings supported by the stack.
    BindingTableSize = 0x04,
    /// The maximum number of EUI64 to network address associations that the stack can maintain
    /// for the application.
    ///
    /// Note, the total number of such address associations maintained by the NCP is the sum of
    /// the value of this setting and the value of `EZSP_CONFIG_TRUST_CENTER_ADDRESS_CACHE_SIZE`.
    AddressTableSize = 0x05,
    /// The maximum number of multicast groups that the device may be a member of.
    MulticastTableSize = 0x06,
    /// The maximum number of destinations to which a node  can route messages.
    ///
    /// This includes both messages originating at this node and those relayed for others.
    RouteTableSize = 0x07,
    /// The number of simultaneous route discoveries that a node will support.
    DiscoveryTableSize = 0x08,
    /// Specifies the stack profile.
    StackProfile = 0x0C,
    /// The security level used for security at the MAC and network layers.
    ///
    /// The supported values are 0 (no security)
    /// and 5 (payload is encrypted and a four-byte MIC is used for authentication).
    SecurityLevel = 0x0D,
    /// The maximum number of hops for a message.
    MaxHops = 0x10,
    /// The maximum number of end device children that a router will support.
    MaxEndDeviceChildren = 0x11,
    /// The maximum amount of time that the MAC will hold a message
    /// for indirect transmission to a child.
    IndirectTransmissionTimeout = 0x12,
    /// The maximum amount of time that an end device child can wait between polls.
    ///
    /// If no poll is heard within this timeout, then the parent removes the end device from its tables.
    /// Value range 0-14. The timeout corresponding to a value of zero is 10 seconds.
    /// The timeout corresponding to a nonzero value N is 2^N minutes,
    /// ranging from 2^1 = 2 minutes to 2^14 = 16384 minutes.
    EndDevicePollTimeout = 0x13,
    /// Enables boost power mode and/or the alternate transmitter output.
    TxPowerMode = 0x17,
    /// * `0`: Allow this node to relay messages.
    /// * `1`: Prevent this node from relaying messages.
    DisableRelay = 0x18,
    /// The maximum number of EUI64 to network address associations that the Trust Center can maintain.
    ///
    /// These address cache entries are reserved for and reused by the Trust Center when processing
    /// device join/rejoin authentications.
    /// This cache size limits the number of overlapping joins the Trust Center can process
    /// within a narrow time window (e.g. two seconds),
    /// and thus should be set to the maximum number of near simultaneous joins
    /// the Trust Center is expected to accommodate.
    ///
    /// Note, the total number of such address associations maintained by the NCP is the sum of
    /// the value of this setting and the value of `EZSP_CONFIG_ADDRESS_TABLE_SIZE`.
    TrustCenterAddressCacheSize = 0x19,
    /// The size of the source route table.
    SourceRouteTableSize = 0x1A,
    /// The time the coordinator will wait for a second end device bind request to arrive.
    EndDevicePollTimeoutShift = 0x1B,
    /// The number of blocks of a fragmented message that can be sent in a single window.
    FragmentWindowSize = 0x1C,
    /// The time the stack will wait (in milliseconds) between sending blocks
    /// of a fragmented message.
    FragmentDelayMs = 0x1D,
    /// The size of the Key Table used for storing individual link keys
    /// (if the device is a Trust Center) or Application Link Keys (if the device is a normal node).
    KeyTableSize = 0x1E,
    /// The APS ACK timeout value. The stack waits this amount of
    /// time between resends of APS retried messages.
    ApsAckTimeout = 0x1F,
    /// The duration of a beacon jitter, in the units used by the 15.4 scan parameter
    /// (((1 << duration) + 1) * 15ms), when responding to a beacon request.
    BeaconJitterDuration = 0x20,
    /// The number of PAN id conflict reports that must be received by
    /// the network manager within one minute to trigger a PAN id change.
    PanIdReportConflictThreshold = 0x22,
    /// The timeout value in minutes for how long the Trust Center or a normal node waits for
    /// the ZigBee Request Key to complete.
    ///
    /// On the Trust Center this controls whether the device buffers the request,
    /// waiting for a matching pair of ZigBee Request Key.
    /// If the value is non-zero, the Trust Center buffers and waits for that amount of time.
    /// If the value is zero, the Trust Center does not buffer the request and
    /// immediately responds to the request.
    /// Zero is the most compliant behavior.
    RequestKeyTimeout = 0x24,
    /// This value indicates the size of the runtime modifiable certificate table.
    ///
    /// Normally certificates are stored in MFG tokens but this table can be used to
    /// field upgrade devices with new Smart Energy certificates.
    ///
    /// This value cannot be set, it can only be queried.
    CertificateTableSize = 0x29,
    /// This is a bitmask that controls which incoming ZDO request messages are passed to
    /// the application.
    ///
    /// The bits are defined in the EmberZdoConfigurationFlags enumeration.
    /// To see if the application is required to send a ZDO response in reply to an incoming message,
    /// the application must check the APS options bitfield within the `incomingMessageHandler`
    /// callback to see if the `EMBER_APS_OPTION_ZDO_RESPONSE_REQUIRED` flag is set.
    ApplicationZdoFlags = 0x2A,
    /// The maximum number of broadcasts during a single broadcast timeout period.
    BroadcastTableSize = 0x2B,
    /// The size of the MAC filter list table.
    MacFilterTableSize = 0x2C,
    /// The number of supported networks.
    SupportedNetworks = 0x2D,
    /// Whether multicasts are sent to the RxOnWhenIdle=true address (`0xFFFD`)
    /// or the sleepy broadcast address (`0xFFFF`).
    ///
    /// The RxOnWhenIdle=true address is the ZigBee compliant destination for multicasts.
    SendMulticastsToSleepyAddress = 0x2E,
    /// ZLL group address initial configuration.
    ZllGroupAddresses = 0x2F,
    /// ZLL rssi threshold initial configuration.
    ZllRssiThreshold = 0x30,
    /// Toggles the `MTORR` flow control in the stack.
    MTorrFlowControl = 0x33,
    /// Setting the retry queue size. Applies to all queues.
    ///
    /// Default value in the sample applications is 16.
    RetryQueueSize = 0x34,
    /// Setting the new broadcast entry threshold.
    ///
    /// The number(`BROADCAST_TABLE_SIZE` - `NEW_BROADCAST_ENTRY_THRESHOLD`) of
    /// broadcast table entries are reserved for relaying the broadcast messages originated
    /// on other devices.
    /// The local device will fail to originate a broadcast message after this threshold is reached.
    /// Setting this value to `BROADCAST_TABLE_SIZE` and greater
    /// will effectively kill this limitation.
    NewBroadcastEntryThreshold = 0x35,
    /// The length of time, in seconds, that a trust center will store a transient link key
    /// that a device can use to join its network.
    ///
    /// A transient key is added with a call to `emberAddTransientLinkKey`.
    /// After the transient key is added, it will be removed once this amount of time has passed.
    /// A joining device will not be able to use that key to join until
    /// it is added again on the trust center.
    /// The default value is 300 seconds, i.e., 5 minutes.
    TransientKeyTimeoutSec = 0x36,
    /// The number of passive acknowledgements to record from neighbors
    /// before we stop re-transmitting broadcasts.
    BroadcastMinAcksNeeded = 0x37,
    /// The length of time, in seconds, that a trust center will allow a
    /// Trust Center (insecure) rejoin for a device that is using the well-known link key.
    ///
    /// This timeout takes effect once rejoins using the well-known key has been allowed.
    /// This command updates the `sli_zigbee_allow_tc_rejoins_using_well_known_key_timeout_sec`
    /// value.
    TcRejoinsUsingWellKnownKeyTimeoutSec = 0x38,
    /// Valid range of a `CTUNE` value is `0x0000` - `0x01FF`.
    ///
    /// Higher order bits (0xFE00) of the 16-bit value are ignored.
    CTuneValue = 0x39,
    /// To configure non trust center node to assume a concentrator type of the trust center
    /// it join to, until it receive many-to-one route request from the trust center.
    ///
    /// For the trust center node, concentrator type is configured from the concentrator plugin.
    /// The stack by default assumes trust center be a low RAM concentrator that make other devices
    /// send route record to the trust center even without receiving a many-to-one route request.
    ///
    /// The default concentrator type can be changed by setting appropriate
    /// `EmberAssumeTrustCenterConcentratorType` config value.
    AssumeTcConcentratorType = 0x40,
    /// This is green power proxy table size.
    ///
    /// This value is read-only and cannot be set at runtime.
    GpProxyTableSize = 0x41,
    /// This is green power sink table size.
    ///
    /// This value is read-only and cannot be set at runtime.
    GpSinkTableSize = 0x42,
}

impl From<Id> for u8 {
    fn from(id: Id) -> Self {
        id as Self
    }
}

impl TryFrom<u8> for Id {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
