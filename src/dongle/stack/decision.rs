use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Decision {
    DeferJoinsRejoinsHaveLinkKey = 0x07,
    DisallowBindingModification = 0x10,
    AllowBindingModification = 0x11,
    CheckBindingModificationsAreValidEndpointClusters = 0x12,
    HostWillNotSupplyReply = 0x20,
    HostWillSupplyReply = 0x21,
    PollHandlerIgnore = 0x30,
    PollHandlerCallback = 0x31,
    MessageTagOnlyInCallback = 0x40,
    MessageTagAndContentsInCallback = 0x41,
    DenyTcKeyRequests = 0x50,
    AllowTcKeyRequestsAndSendCurrentKey = 0x51,
    AllowTcKeyRequestsAndGenerateNewKey = 0x52,
    DenyAppKeyRequests = 0x60,
    AllowAppKeyRequests = 0x61,
    PacketValidateLibraryChecksEnabled = 0x62,
    PacketValidateLibraryChecksDisabled = 0x63,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Bitmask {
    Default = 0x00,
    AllowJoins = 0x01,
    AllowUnsecuredRejoins = 0x02,
    SendKeyInClear = 0x04,
    IgnoreUnsecuredRejoins = 0x08,
    JoinsUseInstallCodeKey = 0x10,
    DeferJoins = 0x20,
}
