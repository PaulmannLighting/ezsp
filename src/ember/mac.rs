use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PassthroughType {
    None = 0x00,
    SeInterPan = 0x01,
    EmberNet = 0x02,
    EmberNetSource = 0x04,
}

impl From<PassthroughType> for u8 {
    fn from(passthrough_type: PassthroughType) -> Self {
        passthrough_type
            .to_u8()
            .expect("could not convert PassthroughType to u8")
    }
}

impl TryFrom<u8> for PassthroughType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
