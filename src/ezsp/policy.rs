//! Policy configuration.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Identifies a policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Id {
    /// Controls trust center behavior.
    TrustCenter = 0x00,
    /// Controls how external binding modification requests are handled.
    BindingModification = 0x01,
    /// Controls whether the Host supplies unicast replies.
    UnicastReplies = 0x02,
    /// Controls whether pollHandler callbacks are generated.
    PollHandler = 0x03,
    /// Controls whether the message contents are included in the `messageSentHandler` callback.
    MessageContentsInCallback = 0x04,
    /// Controls whether the Trust Center will respond to Trust Center link key requests.
    TcKeyRequest = 0x05,
    /// Controls whether the Trust Center will respond to application link key requests.
    KeyRequest = 0x06,
    /// Controls whether `ZigBee` packets that appear invalid are automatically dropped by the stack.
    ///
    /// A counter will be incremented when this occurs.
    PacketValidateLibrary = 0x07,
    /// Controls whether the stack will process ZLL messages.
    Zll = 0x08,
    /// Controls whether Trust Center (insecure) rejoins for devices using the
    /// well-known link key are accepted.
    ///
    /// If rejoining using the well-known key is allowed, it is disabled again after
    /// `sli_zigbee_allow_tc_rejoins_using_well_known_key_timeout_sec` seconds.
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
