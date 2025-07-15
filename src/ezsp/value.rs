//! EZSP value identifiers.

pub use ember_version::EmberVersion;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

mod ember_version;

/// Identifies a value.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Id {
    /// The contents of the node data stack token.
    TokenStackNodeData = 0x00,
    /// The types of MAC passthrough messages that the host wishes to receive.
    MacPassthroughFlags = 0x01,
    /// The source address used to filter legacy `EmberNet` messages when the
    /// [`PassThroughType`](crate::ember::mac::PassThroughType) flag is set in
    /// [`Self::MacPassthroughFlags`].
    EmbernetPassthroughSourceAddress = 0x02,
    /// The number of available internal RAM general purpose buffers. Read only.
    FreeBuffers = 0x03,
    /// Selects sending synchronous callbacks in `EZSP-UART`.
    UartSynchCallbacks = 0x04,
    /// The maximum incoming transfer size for the local node.
    ///
    /// Default value is set to 82 and does not use fragmentation.
    /// Sets the value in Node Descriptor.
    /// To set, this takes the input of a uint8 array of length 2 where you pass the lower byte
    /// at index 0 and upper byte at index 1.
    MaximumIncomingTransferSize = 0x05,
    /// The maximum outgoing transfer size for the local node.
    ///
    /// Default value is set to 82 and does not use fragmentation.
    /// Sets the value in Node Descriptor.
    /// To set, this takes the input of a uint8 array of length 2 where you pass the lower
    /// byte at index 0 and upper byte at index 1.
    MaximumOutgoingTransferSize = 0x06,
    /// A bool indicating whether stack tokens are written to persistent storage as they change.
    StackTokenWriting = 0x07,
    /// A read-only value indicating whether the stack is currently performing a rejoin.
    StackIsPerformingRejoin = 0x08,
    // TODO: where is `EmberMacFilterMatchData` defined?
    /// A list of `EmberMacFilterMatchData` values.
    MacFilterList = 0x09,
    /// The Ember Extended Security Bitmask.
    ExtendedSecurityBitmask = 0x0A,
    /// The node short ID.
    NodeShortId = 0x0B,
    /// The descriptor capability of the local node. Write only.
    DescriptorCapability = 0x0C,
    /// The stack device request sequence number of the local node.
    StackDeviceRequestSequenceNumber = 0x0D,
    /// Enable or disable radio hold-off.
    RadioHoldOff = 0x0E,
    /// The flags field associated with the endpoint data.
    EndpointFlags = 0x0F,
    /// Enable/disable the Mfg security config key settings.
    MfgSecurityConfig = 0x10,
    /// Retrieves the version information from the stack on the NCP.
    VersionInfo = 0x11,
    // TODO: Where is `EMBER_REJOIN_REASON_NONE` defined?
    /// This will get/set the rejoin reason noted by the host for a subsequent call to
    /// [`Networking::find_and_rejoin_network`](crate::Networking::find_and_rejoin_network).
    ///
    /// After a call to `Networking::find_and_rejoin_network`
    /// the host's rejoin reason will be set to `EMBER_REJOIN_REASON_NONE`.
    /// The NCP will store the rejoin reason used by the call to `Networking::find_and_rejoin_network`.
    /// Application is not required to do anything with this value.
    ///
    /// The App Framework sets this for cases of `Networking::find_and_rejoin_network` that it initiates,
    /// but if the app is invoking a rejoin directly, it should/can set this value to aid in
    /// debugging of any rejoin state machine issues over EZSP logs after the fact.
    ///
    /// The NCP doesn't do anything with this value other than  cache it so you can read it later.
    NextHostRejoinReason = 0x12,
    // TODO: Where are `EMBER_REJOIN_DUE_TO_APP_EVENT_1`, `EMBER_REJOIN_DUE_TO_APP_EVENT_X`, etc defined?
    /// This is the reason that the last rejoin took place.
    ///
    /// This value may only be retrieved, not set.
    /// The rejoin may have been initiated by the stack (NCP) or the application (host).
    /// If a host initiated a rejoin the reason will be set by default to
    /// `EMBER_REJOIN_DUE_TO_APP_EVENT_1`.
    ///
    /// If the application wishes to denote its own rejoin reasons it can do so by calling
    /// `ezspSetValue(EMBER_VALUE_HOST_REJOIN_REASON,  EMBER_REJOIN_DUE_TO_APP_EVENT_X)`.
    /// X is a number corresponding to one of the app events defined.
    ///
    /// If the NCP initiated a rejoin it will record this value internally for retrieval by
    /// `ezspGetValue(EZSP_VALUE_REAL_REJOIN_REASON)`.
    LastRejoinReason = 0x13,
    /// The next Zigbee sequence number.
    NextZigbeeSequenceNumber = 0x14,
    /// CCA energy detect threshold for radio.
    CcaThreshold = 0x15,
    /// The threshold value for a counter.
    SetCounterThreshold = 0x017,
    /// Resets all counters thresholds to 0xFF.
    ResetCounterThresholds = 0x18,
    /// Clears all the counters.
    ClearCounters = 0x19,
    /// The node's new certificate signed by the CA..
    Certificate283K1 = 0x1A,
    /// The Certificate Authority's public key.
    PublicKey283K1 = 0x1B,
    /// The node's new static private key.
    PrivateKey283K1 = 0x1C,
    /// The NWK layer security frame counter value.
    NwkFrameCounter = 0x23,
    /// The APS layer security frame counter value.
    ///
    /// Managed by the stack. Users should not set these unless doing backup and restore.
    ApsFrameCounter = 0x24,
    /// Sets the device type to use on the next rejoin using device type.
    RetryDeviceType = 0x25,
    /// Setting this byte enables R21 behavior on the NCP.
    EnableR21Behavior = 0x29,
    /// Configure the antenna mode(0-don't switch, 1-primary, 2-secondary, 3-TX antenna diversity).
    AntennaMode = 0x30,
    /// Enable or disable packet traffic arbitration.
    EnablePta = 0x31,
    /// Set packet traffic arbitration configuration options.
    PtaOptions = 0x32,
    /// Configure manufacturing library options:
    ///
    /// * `0`: non-CSMA transmits
    /// * `1`: CSMA transmits
    ///
    /// To be used with Manufacturing library.
    MfglibOptions = 0x33,
    /// Sets the flag to use either negotiated power by link power delta (LPD) or
    /// fixed power value provided by user while forming/joining a network for packet
    /// transmissions on sub-ghz interface.
    ///
    /// This is mainly for testing purposes.
    UseNegotiatedPowerByLpd = 0x34,
    /// Set packet traffic arbitration PWM options.
    PtaPwmOptions = 0x35,
    /// Set packet traffic arbitration directional priority pulse width in microseconds.
    PtaDirectionalPriorityPulseWidth = 0x36,
    /// Set packet traffic arbitration phy select timeout(ms).
    PtaPhySelectTimeout = 0x37,
    /// Configure the RX antenna mode:
    ///
    /// * `0`: do not switch
    /// * `1`: primary
    /// * `2`: secondary
    /// * `3`: RX antenna diversity
    AntennaRxMode = 0x38,
    /// Configure the timeout to wait for the network key before failing a join.
    ///
    /// Acceptable timeout range `[3,255]`. Value is in seconds.
    NwkKeyTimeout = 0x39,
    /// The number of failed CSMA attempts due to failed CCA made by the MAC before continuing
    /// transmission with CCA disabled.
    ///
    /// This is the same as calling the `emberForceTxAfterFailedCca(uint8_t csmaAttempts)` API.
    /// A value of 0 disables the feature.
    ForceTxAfterFailedCcaAttempts = 0x3A,
    /// The length of time, in seconds, that a trust center will store a transient link key that a
    /// device can use to join its network.
    ///
    /// A transient key is added with a call to `sl_zb_sec_man_import_transient_key`.
    /// After the transient key is added, it will be removed once this amount of time has passed.
    /// A joining device will not be able to use that key to join until it is added again on the
    /// trust center.
    ///
    /// The default value is 300 seconds (5 minutes).
    TransientKeyTimeoutSec = 0x3B,
    /// Cumulative energy usage metric since the last value reset of the coulomb counter plugin.
    ///
    /// Setting this value will reset the coulomb counter.
    CoulombCounterUsage = 0x3C,
    /// When scanning, configure the maximum number of beacons to store in cache.
    ///
    /// Each beacon consumes one packet buffer in RAM.
    MaxBeaconsToStore = 0x3D,
    /// Set the mask to filter out unacceptable child timeout options on a router.
    EndDeviceTimeoutOptionsMask = 0x3E,
    /// The end device keep-alive mode supported by the parent.
    EndDeviceKeepAliveSupportMode = 0x3F,
    /// Return the active radio config. Read only. Values are:
    ///
    /// * `0`: Default
    /// * `1`: Antenna Diversity
    /// * `2`: Co-Existence,
    /// * `3`: Antenna diversity and Co-Existence
    ActiveRadioConfig = 0x41,
    /// Return the number of seconds the network will remain open.
    ///
    /// A return value of 0 indicates that the network is closed. Read only.
    NwkOpenDuration = 0x42,
    /// Timeout in milliseconds to store entries in the transient device table.
    ///
    /// If the devices are not authenticated before the timeout, the entry shall be purged.
    TransientDeviceTimeout = 0x43,
    /// Return information about the key storage on an NCP.
    ///
    /// Returns 0 if keys are in classic key storage, and 1 if they are located in PSA key storage.
    /// Read only.
    KeyStorageVersion = 0x44,
    /// Return activation state about TC Delayed Join on an NCP.
    ///
    /// A return value of 0 indicates that the feature is not activated.
    DelayedJoinActivation = 0x45,
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

/// Identifies a value based on specified characteristics.
///
/// Each set of characteristics is unique to that value and is specified
/// during the call to get the extended value.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum ExtendedId {
    /// The flags field associated with the specified endpoint.
    EndpointFlags = 0x00,
    /// This is the reason for the node to leave the network
    /// as well as the device that told it to leave.
    ///
    /// The leave reason is the 1st byte of the value while the node ID is the 2nd and 3rd byte.
    /// If the leave was caused due to an API call rather than an over the air message,
    /// the node ID will be `EMBER_UNKNOWN_NODE_ID` (0xFFFD).
    LastLeaveReason = 0x01,
    /// This number of bytes of overhead required in the network frame
    /// for source routing to a particular destination.
    GetSourceRouteOverhead = 0x02,
}

impl From<ExtendedId> for u8 {
    fn from(extended_id: ExtendedId) -> Self {
        extended_id as Self
    }
}

impl TryFrom<u8> for ExtendedId {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
