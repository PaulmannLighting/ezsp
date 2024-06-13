use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Id {
    TrustCenter = 0x00,
    BindingModification = 0x01,
    UnicastReplies = 0x02,
    PollHandler = 0x03,
    MessageContentsInCallback = 0x04,
    TcKeyRequest = 0x05,
    KeyRequest = 0x06,
    PacketValidateLibrary = 0x07,
    Zll = 0x08,
    TcJoinsUsingWellKnownKey = 0x09,
}

impl From<Id> for u8 {
    fn from(id: Id) -> Self {
        id as Self
    }
}

impl TryFrom<u8> for Id {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
