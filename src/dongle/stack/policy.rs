use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Policy {
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
