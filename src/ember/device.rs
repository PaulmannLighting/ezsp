//! Ember device module.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember device update.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Update {
    /// A secured rejoin occurred.
    StandardSecuritySecuredRejoin = 0x00,
    /// An unsecured join occurred.
    StandardSecurityUnsecuredJoin = 0x01,
    /// A device left the network.
    DeviceLeft = 0x02,
    /// An unsecured rejoin occurred.
    StandardSecurityUnsecuredRejoin = 0x03,
}

impl From<Update> for u8 {
    fn from(update: Update) -> Self {
        update as Self
    }
}

impl TryFrom<u8> for Update {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
