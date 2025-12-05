//! Policy decision identifiers and bitmasks.

use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;

/// Identifies a policy decision.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Id {
    /// Send the network key in the clear to all joining and rejoining devices.
    AllowJoins = 0x00,
    /// Send the network key encrypted with the joining or rejoining device's trust center link key.
    /// The trust center and any joining or rejoining device are assumed to share a link key, either
    /// preconfigured or obtained under a previous policy. This is the default value for the
    /// [`TrustCenter`](crate::ezsp::policy::Id::TrustCenter).
    AllowPreconfiguredKeyJoins = 0x01,
    /// Send the network key encrypted with the rejoining device's trust center link key. The trust
    /// center and any rejoining device are assumed to share a link key, either preconfigured or
    /// obtained under a previous policy. No new devices are allowed to join.
    AllowRejoinsOnly = 0x02,
    /// Reject all unsecured join and rejoin attempts.
    DisallowAllJoinsAndRejoins = 0x03,
    /// Send the network key in the clear to all joining devices. Rejoining devices are sent the
    /// network key encrypted with their trust center link key. The trust center and any rejoining
    /// device are assumed to share a link key, either preconfigured or obtained under a previous
    /// policy.
    AllowJoinsRejoinsHaveLinkKey = 0x04,
    /// Take no action on trust center rejoin attempts.
    IgnoreTrustCenterRejoins = 0x05,
    /// Admit joins only if there is an entry in the transient key table. This corresponds to the Base
    /// Device Behavior specification where a Trust Center enforces all devices to join with an
    /// install code-derived link key.
    BdbJoinUsesInstallCodeKey = 0x06,
    /// Delay sending the network key to a new joining device.
    DeferJoinsRejoinsHaveLinkKey = 0x07,
    /// [`BindingModification`](crate::ezsp::policy::Id::BindingModification) default decision.
    ///
    /// Do not allow the local binding table to be changed by remote nodes.
    DisallowBindingModification = 0x10,
    /// [`BindingModification`](crate::ezsp::policy::Id::BindingModification) decision.
    ///
    /// Allow remote nodes to change the local binding table.
    AllowBindingModification = 0x11,
    /// [`BindingModification`](crate::ezsp::policy::Id::BindingModification) decision.
    ///
    /// Allows remote nodes to set local binding entries only if the entries correspond to
    /// endpoints defined on the device, and for output clusters bound to those endpoints.
    CheckBindingModificationsAreValidEndpointClusters = 0x12,
    /// [`UnicastReplies`](crate::ezsp::policy::Id::UnicastReplies) default decision.
    ///
    /// The NCP will automatically send an empty reply (containing no payload)
    /// for every unicast received.
    HostWillNotSupplyReply = 0x20,
    /// [`UnicastReplies`](crate::ezsp::policy::Id::UnicastReplies) decision.
    ///
    /// The NCP will only send a reply if it receives a sendReply command from the Host.
    HostWillSupplyReply = 0x21,
    /// [`PollHandler`](crate::ezsp::policy::Id::PollHandler) default decision.
    ///
    /// Do not inform the Host when a child polls.
    PollHandlerIgnore = 0x30,
    /// [`PollHandler`](crate::ezsp::policy::Id::PollHandler) decision.
    ///
    /// Generate a pollHandler callback when a child polls.
    PollHandlerCallback = 0x31,
    /// [`MessageContentsInCallback`](crate::ezsp::policy::Id::MessageContentsInCallback) default decision.
    ///
    /// Include only the message tag in the messageSentHandler callback.
    MessageTagOnlyInCallback = 0x40,
    /// [`MessageContentsInCallback`](crate::ezsp::policy::Id::MessageContentsInCallback) decision.
    ///
    /// Include both the message tag and the message contents in the messageSentHandler callback.
    MessageTagAndContentsInCallback = 0x41,
    /// [`TcKeyRequest`](crate::ezsp::policy::Id::TcKeyRequest) decision.
    ///
    /// When the Trust Center receives a request for a Trust Center link key, it will be ignored.
    DenyTcKeyRequests = 0x50,
    /// [`TcKeyRequest`](crate::ezsp::policy::Id::TcKeyRequest) decision.
    ///
    /// When the Trust Center receives a request for a Trust Center link key,
    /// it will reply to it with the corresponding key.
    AllowTcKeyRequestsAndSendCurrentKey = 0x51,
    /// [`TcKeyRequest`](crate::ezsp::policy::Id::TcKeyRequest) decision.
    ///
    /// When the Trust Center receives a request for a Trust Center link key,
    /// it will generate a key to send to the joiner.
    /// After generation, the key will be added to the transient key table and after
    /// verification this key will be added to the link key table.
    AllowTcKeyRequestsAndGenerateNewKey = 0x52,
    /// [`AppKeyRequest`](crate::ezsp::policy::Id::AppKeyRequest) decision.
    ///
    /// When the Trust Center receives a request for an application link key, it will be ignored.
    DenyAppKeyRequests = 0x60,
    /// [`AppKeyRequest`](crate::ezsp::policy::Id::AppKeyRequest) decision.
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Bitmask(u8);
bitflags! {
    impl Bitmask: u8 {
        /// Disallow joins and rejoins.
        const DEFAULT = 0x00;
        /// Send the network key to all joining devices.
        const ALLOW_JOINS = 0x01;
        /// Send the network key to all rejoining devices.
        const ALLOW_UNSECURED_REJOINS = 0x02;
        /// Send the network key in the clear.
        const SEND_KEY_IN_CLEAR = 0x04;
        /// Do nothing for unsecured rejoins.
        const IGNORE_UNSECURED_REJOINS = 0x08;
        /// Allow joins if there is an entry in the transient key table.
        const JOINS_USE_INSTALL_CODE_KEY = 0x10;
        /// Delay sending the network key to a new joining device.
        const DEFER_JOINS = 0x20;
    }
}

impl From<Id> for Bitmask {
    fn from(value: Id) -> Self {
        Self::from_bits_retain(value.into())
    }
}
