use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Bitmask {
    NoOptions = 0x0000,
    ParentInfoInToken = 0x0001,
    EndDeviceRejoinOnReboot = 0x0002,
}

impl From<Bitmask> for u16 {
    fn from(init_bitmask: Bitmask) -> Self {
        init_bitmask
            .to_u16()
            .expect("InitBitmask should always be convertible to u8.")
    }
}

impl TryFrom<u16> for Bitmask {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
