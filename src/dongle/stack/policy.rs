use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Policy {
    TrustCenter = 0x0000,
    BindingModification = 0x0001,
    UnicastReplies = 0x0002,
    PollHandler = 0x0003,
    MessageContentsInCallback = 0x0004,
    TcKeyRequest = 0x0005,
    KeyRequest = 0x0006,
    PacketValidateLibrary = 0x0007,
    Zll = 0x0008,
    TcJoinsUsingWellKnownKey = 0x0009,
}
