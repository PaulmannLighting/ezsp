use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Update {
    StandardSecuritySecuredRejoin = 0x00,
    StandardSecurityUnsecuredJoin = 0x01,
    DeviceLeft = 0x02,
    StandardSecurityUnsecuredRejoin = 0x03,
}

impl From<Update> for u8 {
    fn from(update: Update) -> Self {
        update
            .to_u8()
            .expect("Update should always be convertible to u8.")
    }
}

impl TryFrom<u8> for Update {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
