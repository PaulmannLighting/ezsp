use crate::ember::types::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Bitmask {
    StandardSecurityMode = 0x0000,
    DistributedTrustCenterMode = 0x0002,
    GlobalLinkKey = 0x0004,
    HaveTrustCenterLinkKey = 0x0010,
    TrustCenterUsesHashedLinkKey = 0x0084,
}

impl From<Bitmask> for u16 {
    fn from(bitmask: Bitmask) -> Self {
        bitmask
            .to_u16()
            .expect("Bitmask should always be convertible to u16.")
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
    trust_center_long_address: Eui64,
}

impl State {
    #[must_use]
    pub fn new(bitmask: Bitmask, trust_center_long_address: Eui64) -> Self {
        Self {
            bitmask: bitmask.into(),
            trust_center_long_address,
        }
    }

    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    #[must_use]
    pub const fn trust_center_long_address(&self) -> Eui64 {
        self.trust_center_long_address
    }
}
