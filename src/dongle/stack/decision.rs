use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Decision {
    AllowJoins = 0x0000,
    AllowPreconfiguredKeyJoins = 0x0001,
    AllowRejoinsOnly = 0x0002,
    DisallowAllJoinsAndRejoins = 0x0003,
    AllowJoinsRejoinsHaveLinkKey = 0x0004,
    IgnoreTrustCenterRejoins = 0x0005,
    BdbJoinUsesInstallCodeKey = 0x0006,
    DeferJoinsRejoinsHaveLinkKey = 0x0007,
    DisallowBindingModification = 0x0010,
    AllowBindingModification = 0x0011,
    CheckBindingModificationsAreValidEndpointClusters = 0x0012,
    HostWillNotSupplyReply = 0x0020,
    HostWillSupplyReply = 0x0021,
    PollHandlerIgnore = 0x0030,
    PollHandlerCallback = 0x0031,
    MessageTagOnlyInCallback = 0x0040,
    MessageTagAndContentsInCallback = 0x0041,
    DenyTcKeyRequests = 0x0050,
    AllowTcKeyRequestsAndSendCurrentKey = 0x0051,
    AllowTcKeyRequestsAndGenerateNewKey = 0x0052,
    DenyAppKeyRequests = 0x0060,
    AllowAppKeyRequests = 0x0061,
    PacketValidateLibraryChecksEnabled = 0x0062,
    PacketValidateLibraryChecksDisabled = 0x0063,
}
