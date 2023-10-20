use anyhow::anyhow;
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
        typ.to_u8().expect("could not convert Type to u8.")
    }
}

impl TryFrom<u8> for Type {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or_else(|| anyhow!("Invalid Type: {value:#04X}"))
    }
}
