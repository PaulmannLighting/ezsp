use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
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
        id.to_u8().expect("could not convert Id to u8")
    }
}
