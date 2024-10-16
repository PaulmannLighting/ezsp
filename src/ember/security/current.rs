//! Ember current security state.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::ember::types::Eui64;

/// Ember current security bitmask.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum Bitmask {
    /// This denotes that the device is running in a network with Zigbee Standard Security.
    StandardSecurityMode = 0x0000,
    /// This denotes that the device is running in a network without a centralized Trust Center.
    DistributedTrustCenterMode = 0x0002,
    /// This denotes that the device has a Global Link Key.
    ///
    /// The Trust Center Link Key is the same across multiple nodes.
    GlobalLinkKey = 0x0004,
    /// This denotes that the node has a Trust Center Link Key.
    HaveTrustCenterLinkKey = 0x0010,
    /// This denotes that the Trust Center is using a Hashed Link Key.
    TrustCenterUsesHashedLinkKey = 0x0084,
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

/// The security options and information currently used by the stack.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct State {
    bitmask: u16,
    trust_center_long_address: Eui64,
}

impl State {
    /// Create a new current security state.
    #[must_use]
    pub fn new(bitmask: Bitmask, trust_center_long_address: Eui64) -> Self {
        Self {
            bitmask: bitmask.into(),
            trust_center_long_address,
        }
    }

    /// Return the bitmask indicating the security options currently in use
    /// by a device joined in the network.
    ///
    /// # Errors
    /// Returns the [`u16`] value of the bitmask if it is not a valid [`Bitmask`].
    pub fn bitmask(&self) -> Result<Bitmask, u16> {
        Bitmask::try_from(self.bitmask)
    }

    /// Return the IEEE Address of the Trust Center device.
    #[must_use]
    pub const fn trust_center_long_address(&self) -> Eui64 {
        self.trust_center_long_address
    }
}
