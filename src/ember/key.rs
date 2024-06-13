use std::num::TryFromIntError;
use std::time::Duration;

use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::ember::types::Eui64;

pub type Data = [u8; 16];

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Type {
    TrustCenterLinkKey = 0x01,
    CurrentNetworkKey = 0x03,
    NextNetworkKey = 0x04,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum Bitmask {
    HasSequenceNumber = 0x00001,
    HasOutgoingFrameCounter = 0x0002,
    HasIncomingFrameCounter = 0x0004,
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }

    #[must_use]
    pub const fn key(&self) -> &Data {
        &self.key
    }

    #[must_use]
    pub const fn outgoing_frame_counter(&self) -> u32 {
        self.outgoing_frame_counter
    }

    #[must_use]
    pub const fn incoming_frame_counter(&self) -> u32 {
        self.incoming_frame_counter
    }

    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }

    #[must_use]
    pub const fn partner_eui64(&self) -> Eui64 {
        self.partner_eui64
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct TransientData {
    eui64: Eui64,
    key_data: Data,
    bitmask: u16,
    remaining_time_seconds: u16,
    network_index: u8,
}

impl TransientData {
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

    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    #[must_use]
    pub const fn data(&self) -> &Data {
        &self.key_data
    }

    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    #[must_use]
    pub const fn remaining_time_seconds(&self) -> u16 {
        self.remaining_time_seconds
    }

    #[must_use]
    pub const fn remaining_time(&self) -> Duration {
        Duration::from_secs(self.remaining_time_seconds as u64)
    }

    #[must_use]
    pub const fn network_index(&self) -> u8 {
        self.network_index
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Status {
    AppLinkKeyEstablished = 0x01,
    TrustCenterLinkKeyEstablished = 0x03,
    KeyEstablishmentTimeout = 0x04,
    KeyTableFull = 0x05,
    Tc(Tc),
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        match status {
            Status::AppLinkKeyEstablished => 0x01,
            Status::TrustCenterLinkKeyEstablished => 0x03,
            Status::KeyEstablishmentTimeout => 0x04,
            Status::KeyTableFull => 0x05,
            Status::Tc(tc) => tc.into(),
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
            n => Tc::from_u64(n).map(Self::Tc),
        }
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Tc {
    RespondedToKeyRequest = 0x06,
    AppKeySentToRequester = 0x07,
    ResponseToKeyRequestFailed = 0x08,
    RequestKeyTypeNotSupported = 0x09,
    NoLinkKeyForRequester = 0x0A,
    RequesterEui64Unknown = 0x0B,
    ReceivedFirstAppKeyRequest = 0x0C,
    TimeoutWaitingForSecondAppKeyRequest = 0x0D,
    NonMatchingAppKeyRequestReceived = 0x0E,
    FailedToSendAppKeys = 0x0F,
    FailedToStoreAppKeyRequest = 0x10,
    RejectedAppKeyRequest = 0x11,
}

impl From<Tc> for u8 {
    fn from(tc: Tc) -> Self {
        tc as Self
    }
}

impl TryFrom<u8> for Tc {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
