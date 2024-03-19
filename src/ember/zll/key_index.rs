use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum KeyIndex {
    Development = 0x00,
    Master = 0x04,
    Certification = 0x0F,
}

impl From<KeyIndex> for u8 {
    fn from(key_index: KeyIndex) -> Self {
        key_index
            .to_u8()
            .expect("KeyIndex should always be convertible to u8.")
    }
}

impl TryFrom<u8> for KeyIndex {
    type Error = u8;

    fn try_from(typ: u8) -> Result<Self, Self::Error> {
        Self::from_u8(typ).ok_or(typ)
    }
}
