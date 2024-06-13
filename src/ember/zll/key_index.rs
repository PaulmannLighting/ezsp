use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum KeyIndex {
    Development = 0x00,
    Master = 0x04,
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
