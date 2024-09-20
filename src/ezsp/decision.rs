use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Identifies a policy decision.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Id {
    /// Delay sending the network key to a new joining device.
    DeferJoinsRejoinsHaveLinkKey = 0x07,
    /// `EZSP_BINDING_MODIFICATION_POLICY` default decision.
    ///
    /// Do not allow the local binding table to be changed by remote nodes.
    DisallowBindingModification = 0x10,
    /// `EZSP_BINDING_MODIFICATION_POLICY` decision.
    ///
    /// Allow remote nodes to change the local binding table.
    AllowBindingModification = 0x11,
    /// `EZSP_BINDING_MODIFICATION_POLICY` decision.
    ///
    /// Allows remote nodes to set local binding entries only if the entries correspond to
    /// endpoints defined on the device, and for output clusters bound to those endpoints.
    CheckBindingModificationsAreValidEndpointClusters = 0x12,
    /// `EZSP_UNICAST_REPLIES_POLICY` default decision.
    ///
    /// The NCP will automatically send an empty reply (containing no payload)
    /// for every unicast received.
    HostWillNotSupplyReply = 0x20,
    /// `EZSP_UNICAST_REPLIES_POLICY` decision.
    ///
    /// The NCP will only send a reply if it receives a sendReply command from the Host.
    HostWillSupplyReply = 0x21,
    /// `EZSP_POLL_HANDLER_POLICY` default decision.
    ///
    /// Do not inform the Host when a child polls.
    PollHandlerIgnore = 0x30,
    /// `EZSP_POLL_HANDLER_POLICY` decision.
    ///
    /// Generate a pollHandler callback when a child polls.
    PollHandlerCallback = 0x31,
    /// `EZSP_MESSAGE_CONTENTS_IN_CALLBACK_POLICY` default decision.
    ///
    /// Include only the message tag in the messageSentHandler callback.
    MessageTagOnlyInCallback = 0x40,
    /// `EZSP_MESSAGE_CONTENTS_IN_CALLBACK_POLICY` decision.
    ///
    /// Include both the message tag and the message contents in the messageSentHandler callback.
    MessageTagAndContentsInCallback = 0x41,
    /// `EZSP_TC_KEY_REQUEST_POLICY` decision.
    ///
    /// When the Trust Center receives a request for a Trust Center link key, it will be ignored.
    DenyTcKeyRequests = 0x50,
    /// `EZSP_TC_KEY_REQUEST_POLICY` decision.
    ///
    /// When the Trust Center receives a request for a Trust Center link key,
    /// it will reply to it with the corresponding key.
    AllowTcKeyRequestsAndSendCurrentKey = 0x51,
    /// `EZSP_TC_KEY_REQUEST_POLICY` decision.
    ///
    /// When the Trust Center receives a request for a Trust Center link key,
    /// it will generate a key to send to the joiner.
    /// After generation, the key will be added to the transient key table and after
    /// verification this key will be added to the link key table.
    AllowTcKeyRequestsAndGenerateNewKey = 0x52,
    /// `EZSP_APP_KEY_REQUEST_POLICY` decision.
    ///
    /// When the Trust Center receives a request for an application link key, it will be ignored.
    DenyAppKeyRequests = 0x60,
    /// `EZSP_APP_KEY_REQUEST_POLICY` decision.
    ///
    /// When the Trust Center receives a request for an application link key,
    /// it will randomly generate a key and send it to both partners.
    AllowAppKeyRequests = 0x61,
    /// Indicates that packet validate library checks are enabled on the NCP.
    PacketValidateLibraryChecksEnabled = 0x62,
    /// Indicates that packet validate library checks are NOT enabled on the NCP.
    PacketValidateLibraryChecksDisabled = 0x63,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Bitmask {
    Default = 0x00,
    AllowJoins = 0x01,
    AllowUnsecuredRejoins = 0x02,
    SendKeyInClear = 0x04,
    IgnoreUnsecuredRejoins = 0x08,
    JoinsUseInstallCodeKey = 0x10,
    DeferJoins = 0x20,
}

impl From<Bitmask> for u8 {
    fn from(bitmask: Bitmask) -> Self {
        bitmask as Self
    }
}

impl TryFrom<u8> for Bitmask {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
