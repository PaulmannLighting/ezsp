use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::ember::key::Data;
use crate::ember::Eui64;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum Bitmask {
    StandardSecurityMode = 0x0000,
    DistributedTrustCenterMode = 0x0002,
    TrustCenterGlobalLinkKey = 0x0004,
    PreconfiguredNetworkKeyMode = 0x0008,
    TrustCenterUsesHashedLinkKey = 0x0084,
    HavePreconfiguredKey = 0x0100,
    HaveNetworkKey = 0x0200,
    GetLinkKeyWhenJoining = 0x0400,
    RequireEncryptedKey = 0x0800,
    NoFrameCounterReset = 0x1000,
    GetPreconfiguredKeyFromInstallCode = 0x2000,
    HaveTrustCenterEui64 = 0x0040,
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

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct State {
    bitmask: u16,
    preconfigured_key: Data,
    network_key: Data,
    network_key_sequence_number: u8,
    preconfigured_trust_center_eui64: Eui64,
}

impl State {
    pub fn new(
        bitmask: Bitmask,
        preconfigured_key: Data,
        network_key: Data,
        network_key_sequence_number: u8,
        preconfigured_trust_center_eui64: Eui64,
    ) -> Self {
        Self {
            bitmask: bitmask.into(),
            preconfigured_key,
            network_key,
            network_key_sequence_number,
            preconfigured_trust_center_eui64,
        }
    }

    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    #[must_use]
    pub const fn preconfigured_key(&self) -> &Data {
        &self.preconfigured_key
    }

    #[must_use]
    pub const fn network_key(&self) -> &Data {
        &self.network_key
    }

    #[must_use]
    pub const fn network_key_sequence_number(&self) -> u8 {
        self.network_key_sequence_number
    }

    #[must_use]
    pub const fn preconfigured_trust_center_eui64(&self) -> Eui64 {
        self.preconfigured_trust_center_eui64
    }
}
