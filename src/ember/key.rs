//! Ember key types and structures.

use std::num::TryFromIntError;
use std::time::Duration;

use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::ember::types::Eui64;

/// Type alias for Ember key data.
pub type Data = [u8; 16];

/// Ember key type.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Type {
    /// A shared key between the Trust Center and a device.
    TrustCenterLinkKey = 0x01,
    /// The current active Network Key used by all devices in the network.
    CurrentNetworkKey = 0x03,
    /// The alternate Network Key that was previously in use, or the newer key that will be switched to.
    NextNetworkKey = 0x04,
    /// An Application Link Key shared with another (non-Trust Center) device.
    ApplicationLinkKey = 0x05,
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

/// Ember key struct bitmask.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum Bitmask {
    /// The key has a sequence number associated with it.
    HasSequenceNumber = 0x00001,
    /// The key has an outgoing frame counter associated with it.
    HasOutgoingFrameCounter = 0x0002,
    /// The key has an incoming frame counter associated with it.
    HasIncomingFrameCounter = 0x0004,
    /// The key has a Partner IEEE address associated with it.
    HasPartnerEui64 = 0x0008,
}

impl From<Bitmask> for u16 {
    fn from(bitmask: Bitmask) -> Self {
        bitmask as Self
    }
}

impl TryFrom<u16> for Bitmask {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

/// A structure containing a key and its associated data.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Struct {
    bitmask: u16,
    typ: u8,
    key: Data,
    outgoing_frame_counter: u32,
    incoming_frame_counter: u32,
    sequence_number: u8,
    partner_eui64: Eui64,
}

impl Struct {
    /// Creates a new Ember key struct.
    #[must_use]
    pub fn new(
        bitmask: Bitmask,
        typ: Type,
        key: Data,
        outgoing_frame_counter: u32,
        incoming_frame_counter: u32,
        sequence_number: u8,
        partner_eui64: Eui64,
    ) -> Self {
        Self {
            bitmask: bitmask.into(),
            typ: typ.into(),
            key,
            outgoing_frame_counter,
            incoming_frame_counter,
            sequence_number,
            partner_eui64,
        }
    }

    /// Return a bitmask indicating the presence of data within the various fields in the structure.
    ///
    /// # Errors
    /// Returns the number of the bitmask if the bitmask is invalid.
    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    /// Return the type of the key.
    ///
    /// # Errors
    /// Returns the number of the type if the type is invalid.
    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }

    /// Return the actual key data.
    #[must_use]
    pub const fn key(&self) -> &Data {
        &self.key
    }

    /// Return the outgoing frame counter associated with the key.
    #[must_use]
    pub const fn outgoing_frame_counter(&self) -> u32 {
        self.outgoing_frame_counter
    }

    /// Return the frame counter of the partner device associated with the key.
    #[must_use]
    pub const fn incoming_frame_counter(&self) -> u32 {
        self.incoming_frame_counter
    }

    /// Return the sequence number associated with the key.
    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }

    /// Return the IEEE address of the partner device also in possession of the key.
    #[must_use]
    pub const fn partner_eui64(&self) -> Eui64 {
        self.partner_eui64
    }
}

/// The transient key data structure.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct TransientData {
    eui64: Eui64,
    key_data: Data,
    bitmask: u16,
    remaining_time_seconds: u16,
    network_index: u8,
}

impl TransientData {
    /// Create new transient data.
    #[must_use]
    pub fn new(
        eui64: Eui64,
        key_data: Data,
        bitmask: Bitmask,
        remaining_time_seconds: u16,
        network_index: u8,
    ) -> Self {
        Self {
            eui64,
            key_data,
            bitmask: bitmask.into(),
            remaining_time_seconds,
            network_index,
        }
    }

    /// Try to create a new transient data.
    ///
    /// # Errors
    /// Returns a [`TryFromIntError`] if the remaining time is too large.
    pub fn try_new(
        eui64: Eui64,
        key_data: Data,
        bitmask: Bitmask,
        remaining_time: Duration,
        network_index: u8,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            eui64,
            key_data,
            bitmask: bitmask.into(),
            remaining_time_seconds: remaining_time.as_secs().try_into()?,
            network_index,
        })
    }

    /// Return the IEEE address paired with the transient link key.
    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    /// Return the key data structure matching the transient key.
    #[must_use]
    pub const fn data(&self) -> &Data {
        &self.key_data
    }

    /// Returns the bitmask.
    ///
    /// This bitmask indicates whether various fields in the structure contain valid data.
    ///
    /// # Errors
    /// Returns the number of the bitmask if the bitmask is invalid.
    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    /// Return the number of seconds remaining before the key is automatically
    /// timed out of the transient key table.
    #[must_use]
    pub const fn remaining_time_seconds(&self) -> u16 {
        self.remaining_time_seconds
    }

    /// Return the time remaining before the key is automatically
    /// timed out of the transient key table.
    #[must_use]
    pub const fn remaining_time(&self) -> Duration {
        Duration::from_secs(self.remaining_time_seconds as u64)
    }

    /// Return the network index indicates which NWK uses this key.
    #[must_use]
    pub const fn network_index(&self) -> u8 {
        self.network_index
    }
}

/// Ember key status.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Status {
    /// The application link key has been established.
    AppLinkKeyEstablished = 0x01,
    /// The Trust Center link key has been established.
    TrustCenterLinkKeyEstablished = 0x03,
    /// The key establishment process has timed out.
    KeyEstablishmentTimeout = 0x04,
    /// The key table is full.
    KeyTableFull = 0x05,
    /// Trust Center status.
    TrustCenter(TrustCenter),
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        match status {
            Status::AppLinkKeyEstablished => 0x01,
            Status::TrustCenterLinkKeyEstablished => 0x03,
            Status::KeyEstablishmentTimeout => 0x04,
            Status::KeyTableFull => 0x05,
            Status::TrustCenter(trust_center) => trust_center.into(),
        }
    }
}

impl FromPrimitive for Status {
    fn from_i64(n: i64) -> Option<Self> {
        Self::from_u64(n.try_into().ok()?)
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x01 => Some(Self::AppLinkKeyEstablished),
            0x03 => Some(Self::TrustCenterLinkKeyEstablished),
            0x04 => Some(Self::KeyEstablishmentTimeout),
            0x05 => Some(Self::KeyTableFull),
            n => TrustCenter::from_u64(n).map(Self::TrustCenter),
        }
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

/// Ember key Trust Center status.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum TrustCenter {
    /// The Trust Center has responded to request for an application key.
    RespondedToKeyRequest = 0x06,
    /// The Trust Center has sent the application key to the requester.
    AppKeySentToRequester = 0x07,
    /// The Trust Center has failed to respond to a request for an application key.
    ResponseToKeyRequestFailed = 0x08,
    /// The Trust Center does not support the key type requested.
    RequestKeyTypeNotSupported = 0x09,
    /// The Trust Center does not have a link key for the requester.
    NoLinkKeyForRequester = 0x0A,
    /// The Trust Center does not have a EUI64 for the requester.
    RequesterEui64Unknown = 0x0B,
    /// The Trust Center has received the first application key request.
    ReceivedFirstAppKeyRequest = 0x0C,
    /// The Trust Center has timed out waiting for the second application key request.
    TimeoutWaitingForSecondAppKeyRequest = 0x0D,
    /// The Trust Center has received a request for an application key that does not match the key it is expecting.
    NonMatchingAppKeyRequestReceived = 0x0E,
    /// The Trust Center has failed to send the application keys.
    FailedToSendAppKeys = 0x0F,
    /// The Trust Center has failed to store the application key request.
    FailedToStoreAppKeyRequest = 0x10,
    /// The Trust Center has rejected the application key request.
    RejectedAppKeyRequest = 0x11,
}

impl From<TrustCenter> for u8 {
    fn from(tc: TrustCenter) -> Self {
        tc as Self
    }
}

impl TryFrom<u8> for TrustCenter {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
