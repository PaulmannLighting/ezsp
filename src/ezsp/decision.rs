//! Policy decision identifiers and bitmasks.

use num_derive::FromPrimitive;

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

/// This is the policy decision bitmask that controls the trust center decision strategies.
///
/// The bitmask is modified and extracted from the [`Id`] for supporting bitmask operations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum Bitmask {
    /// Disallow joins and rejoins.
    Default = 0x0000,
    /// Send the network key to all joining devices.
    AllowJoins = 0x0001,
    /// Send the network key to all rejoining devices.
    AllowUnsecuredRejoins = 0x0002,
    /// Send the network key in the clear.
    SendKeyInClear = 0x0004,
    /// Do nothing for unsecured rejoins.
    IgnoreUnsecuredRejoins = 0x0008,
    /// Allow joins if there is an entry in the transient key table.
    JoinsUseInstallCodeKey = 0x0010,
    /// Delay sending the network key to a new joining device.
    DeferJoins = 0x0020,
}

impl From<Bitmask> for u16 {
    fn from(bitmask: Bitmask) -> Self {
        bitmask as Self
    }
}
