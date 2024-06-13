use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Units {
    Inactive = 0x00,
    MsTime = 0x01,
    QsTime = 0x02,
    MinuteTime = 0x03,
}

impl From<Units> for u8 {
    fn from(units: Units) -> Self {
        units as Self
    }
}

impl TryFrom<u8> for Units {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
