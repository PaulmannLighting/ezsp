use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Config {
    BroadcastSupport = 0x01,
}

impl From<Config> for u8 {
    fn from(config: Config) -> Self {
        config.to_u8().expect("could not convert Config to u8")
    }
}

impl TryFrom<u8> for Config {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
