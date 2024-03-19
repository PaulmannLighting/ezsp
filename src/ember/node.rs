use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Type {
    UnknownDevice = 0x00,
    Coordinator = 0x01,
    Router = 0x02,
    EndDevice = 0x03,
    SleepyEndDevice = 0x04,
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ.to_u8()
            .expect("Type should always be convertible to u8.")
    }
}

impl TryFrom<u8> for Type {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
