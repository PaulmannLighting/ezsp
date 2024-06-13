use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum PassthroughType {
    None = 0x00,
    SeInterPan = 0x01,
    EmberNet = 0x02,
    EmberNetSource = 0x04,
}

impl From<PassthroughType> for u8 {
    fn from(passthrough_type: PassthroughType) -> Self {
        passthrough_type as Self
    }
}

impl TryFrom<u8> for PassthroughType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
