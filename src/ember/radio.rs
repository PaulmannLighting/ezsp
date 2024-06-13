use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum PowerMode {
    RxOn = 0x00,
    Off = 0x01,
}

impl From<PowerMode> for u8 {
    fn from(power_mode: PowerMode) -> Self {
        power_mode as Self
    }
}

impl TryFrom<u8> for PowerMode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
