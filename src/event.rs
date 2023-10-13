use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Units {
    Inactive = 0x00,
    MsTime = 0x01,
    QsTime = 0x02,
    MinuteTime = 0x03,
}

impl From<Units> for u8 {
    fn from(units: Units) -> Self {
        units.to_u8().expect("could not convert Units to u8")
    }
}
