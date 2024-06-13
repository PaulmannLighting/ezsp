use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Config {
    BroadcastSupport = 0x01,
}

impl From<Config> for u8 {
    fn from(config: Config) -> Self {
        config as Self
    }
}

impl TryFrom<u8> for Config {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
