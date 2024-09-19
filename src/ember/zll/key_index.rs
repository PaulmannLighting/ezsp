use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember ZLL key index.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum KeyIndex {
    /// Key encryption algorithm for use during development.
    Development = 0x00,
    /// Key encryption algorithm shared by all certified devices.
    Master = 0x04,
    /// Key encryption algorithm for use during development and certification.
    Certification = 0x0F,
}

impl From<KeyIndex> for u8 {
    fn from(key_index: KeyIndex) -> Self {
        key_index as Self
    }
}

impl TryFrom<u8> for KeyIndex {
    type Error = u8;

    fn try_from(typ: u8) -> Result<Self, Self::Error> {
        Self::from_u8(typ).ok_or(typ)
    }
}
