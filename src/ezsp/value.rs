use super::error::Error;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Id {
    TokenStackNodeData = 0x00,
    MacPassthroughFlags = 0x01,
    EmbernetPassthroughSourceAddress = 0x02,
    FreeBuffers = 0x03,
    UartSynchCallbacks = 0x04,
    MaximumIncomingTransferSize = 0x05,
    MaximumOutgoingTransferSize = 0x06,
    StackTokenWriting = 0x07,
    StackIsPerformingRejoin = 0x08,
    MacFilterList = 0x09,
    ExtendedSecurityBitmask = 0x0A,
    NodeShortId = 0x0B,
    DescriptorCapability = 0x0C,
    StackDeviceRequestSequenceNumber = 0x0D,
    RadioHoldOff = 0x0E,
    EndpointFlags = 0x0F,
    MfgSecurityConfig = 0x10,
    VersionInfo = 0x11,
    NextHostRejoinReason = 0x12,
    LastRejoinReason = 0x13,
    NextZigbeeSequenceNumber = 0x14,
    CcaThreshold = 0x15,
    SetCounterThreshold = 0x017,
    ResetCounterThresholds = 0x18,
    ClearCounters = 0x19,
    Certificate283K1 = 0x1A,
    PublicKey283K1 = 0x1B,
    PrivateKey283K1 = 0x1C,
    NwkFrameCounter = 0x23,
    ApsFrameCounter = 0x24,
    RetryDeviceType = 0x25,
    EnableR21Behavior = 0x29,
    AntennaMode = 0x30,
    EnablePta = 0x31,
    PtaOptions = 0x32,
    MfglibOptions = 0x33,
    UseNegotiatedPowerByLpd = 0x34,
    PtaPwmOptions = 0x35,
    PtaDirectionalPriorityPulseWidth = 0x36,
    PtaPhySelectTimeout = 0x37,
    AntennaRxMode = 0x38,
    NwkKeyTimeout = 0x39,
    ForceTxAfterFailedCcaAttempts = 0x3A,
    TransientKeyTimeoutSec = 0x3B,
    CoulombCounterUsage = 0x3C,
    MaxBeaconsToStore = 0x3D,
    EndDeviceTimeoutOptionsMask = 0x3E,
    EndDeviceKeepAliveSupportMode = 0x3F,
    ActiveRadioConfig = 0x41,
    NwkOpenDuration = 0x42,
    TransientDeviceTimeout = 0x43,
    KeyStorageVersion = 0x44,
}

impl From<Id> for u8 {
    fn from(id: Id) -> Self {
        id.to_u8().expect("could not convert Id to u8")
    }
}

impl TryFrom<u8> for Id {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(Error::InvalidValueId(value))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum ExtendedId {
    EndpointFlags = 0x00,
    LastLeaveReason = 0x01,
    GetSourceRouteOverhead = 0x02,
}

impl From<ExtendedId> for u8 {
    fn from(extended_id: ExtendedId) -> Self {
        extended_id
            .to_u8()
            .expect("could not convert ExtendedId to u8")
    }
}

impl TryFrom<u8> for ExtendedId {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(Error::InvalidExtendedId(value))
    }
}
