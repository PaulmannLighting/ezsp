use crate::frame::parameters::{
    binding, bootloader, cbke, green_power, messaging, mfglib, networking, security, trust_center,
    utilities, zll,
};

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Binding handlers.
    Binding(binding::handler::Handler),
    /// Bootloader handlers.
    Bootloader(bootloader::handler::Handler),
    /// Certificate-based key exchange handlers.
    CertificateBasedKeyExchange(cbke::handler::Handler),
    /// Green Power handlers.
    GreenPower(green_power::handler::Handler),
    /// Messaging handlers.
    Messaging(messaging::handler::Handler),
    /// `Mfglib` handlers.
    MfgLib(mfglib::handler::Handler),
    /// Networking handlers.
    Networking(networking::handler::Handler),
    /// Security handlers.
    Security(security::handler::Handler),
    /// Trust Center handlers.
    TrustCenter(trust_center::handler::Handler),
    /// Utilities handlers.
    Utilities(utilities::handler::Handler),
    /// ZLL handlers.
    Zll(zll::handler::Handler),
}

impl From<binding::handler::RemoteDeleteBinding> for Handler {
    fn from(handler: binding::handler::RemoteDeleteBinding) -> Self {
        Self::Binding(handler.into())
    }
}

impl From<binding::handler::RemoteSetBinding> for Handler {
    fn from(handler: binding::handler::RemoteSetBinding) -> Self {
        Self::Binding(handler.into())
    }
}

impl From<bootloader::handler::BootloadTransmitComplete> for Handler {
    fn from(handler: bootloader::handler::BootloadTransmitComplete) -> Self {
        Self::Bootloader(handler.into())
    }
}

impl From<bootloader::handler::IncomingBootloadMessage> for Handler {
    fn from(handler: bootloader::handler::IncomingBootloadMessage) -> Self {
        Self::Bootloader(handler.into())
    }
}

impl From<cbke::handler::CalculateSmacs> for Handler {
    fn from(handler: cbke::handler::CalculateSmacs) -> Self {
        Self::CertificateBasedKeyExchange(handler.into())
    }
}

impl From<cbke::handler::CalculateSmacs283k1> for Handler {
    fn from(handler: cbke::handler::CalculateSmacs283k1) -> Self {
        Self::CertificateBasedKeyExchange(handler.into())
    }
}

impl From<cbke::handler::DsaSign> for Handler {
    fn from(handler: cbke::handler::DsaSign) -> Self {
        Self::CertificateBasedKeyExchange(handler.into())
    }
}

impl From<cbke::handler::DsaVerify> for Handler {
    fn from(handler: cbke::handler::DsaVerify) -> Self {
        Self::CertificateBasedKeyExchange(handler.into())
    }
}

impl From<cbke::handler::GenerateCbkeKeys> for Handler {
    fn from(handler: cbke::handler::GenerateCbkeKeys) -> Self {
        Self::CertificateBasedKeyExchange(handler.into())
    }
}

impl From<cbke::handler::GenerateCbkeKeys283k1> for Handler {
    fn from(handler: cbke::handler::GenerateCbkeKeys283k1) -> Self {
        Self::CertificateBasedKeyExchange(handler.into())
    }
}

impl From<green_power::handler::IncomingMessage> for Handler {
    fn from(handler: green_power::handler::IncomingMessage) -> Self {
        Self::GreenPower(handler.into())
    }
}

impl From<green_power::handler::Sent> for Handler {
    fn from(handler: green_power::handler::Sent) -> Self {
        Self::GreenPower(handler.into())
    }
}

impl From<messaging::handler::IdConflict> for Handler {
    fn from(handler: messaging::handler::IdConflict) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::IncomingManyToOneRouteRequest> for Handler {
    fn from(handler: messaging::handler::IncomingManyToOneRouteRequest) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::IncomingMessage> for Handler {
    fn from(handler: messaging::handler::IncomingMessage) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::IncomingNetworkStatus> for Handler {
    fn from(handler: messaging::handler::IncomingNetworkStatus) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::IncomingRouteError> for Handler {
    fn from(handler: messaging::handler::IncomingRouteError) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::IncomingSenderEui64> for Handler {
    fn from(handler: messaging::handler::IncomingSenderEui64) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::MacFilterMatchMessage> for Handler {
    fn from(handler: messaging::handler::MacFilterMatchMessage) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::MacPassthroughMessage> for Handler {
    fn from(handler: messaging::handler::MacPassthroughMessage) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::MessageSent> for Handler {
    fn from(handler: messaging::handler::MessageSent) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::Poll> for Handler {
    fn from(handler: messaging::handler::Poll) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::PollComplete> for Handler {
    fn from(handler: messaging::handler::PollComplete) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<messaging::handler::RawTransmitComplete> for Handler {
    fn from(handler: messaging::handler::RawTransmitComplete) -> Self {
        Self::Messaging(handler.into())
    }
}

impl From<mfglib::handler::Rx> for Handler {
    fn from(handler: mfglib::handler::Rx) -> Self {
        Self::MfgLib(handler.into())
    }
}

impl From<networking::handler::ChildJoin> for Handler {
    fn from(handler: networking::handler::ChildJoin) -> Self {
        Self::Networking(handler.into())
    }
}

impl From<networking::handler::DutyCycle> for Handler {
    fn from(handler: networking::handler::DutyCycle) -> Self {
        Self::Networking(handler.into())
    }
}

impl From<networking::handler::EnergyScanResult> for Handler {
    fn from(handler: networking::handler::EnergyScanResult) -> Self {
        Self::Networking(handler.into())
    }
}

impl From<networking::handler::NetworkFound> for Handler {
    fn from(handler: networking::handler::NetworkFound) -> Self {
        Self::Networking(handler.into())
    }
}

impl From<networking::handler::ScanComplete> for Handler {
    fn from(handler: networking::handler::ScanComplete) -> Self {
        Self::Networking(handler.into())
    }
}

impl From<networking::handler::StackStatus> for Handler {
    fn from(handler: networking::handler::StackStatus) -> Self {
        Self::Networking(handler.into())
    }
}

impl From<networking::handler::UnusedPanIdFound> for Handler {
    fn from(handler: networking::handler::UnusedPanIdFound) -> Self {
        Self::Networking(handler.into())
    }
}

impl From<security::handler::SwitchNetworkKey> for Handler {
    fn from(handler: security::handler::SwitchNetworkKey) -> Self {
        Self::Security(handler.into())
    }
}

impl From<security::handler::ZigbeeKeyEstablishment> for Handler {
    fn from(handler: security::handler::ZigbeeKeyEstablishment) -> Self {
        Self::Security(handler.into())
    }
}

impl From<trust_center::handler::TrustCenterJoin> for Handler {
    fn from(handler: trust_center::handler::TrustCenterJoin) -> Self {
        Self::TrustCenter(handler.into())
    }
}

impl From<utilities::handler::CounterRollover> for Handler {
    fn from(handler: utilities::handler::CounterRollover) -> Self {
        Self::Utilities(handler.into())
    }
}

impl From<utilities::handler::CustomFrame> for Handler {
    fn from(handler: utilities::handler::CustomFrame) -> Self {
        Self::Utilities(handler.into())
    }
}

impl From<utilities::handler::StackTokenChanged> for Handler {
    fn from(handler: utilities::handler::StackTokenChanged) -> Self {
        Self::Utilities(handler.into())
    }
}

impl From<utilities::handler::Timer> for Handler {
    fn from(handler: utilities::handler::Timer) -> Self {
        Self::Utilities(handler.into())
    }
}

impl From<zll::handler::AddressAssignment> for Handler {
    fn from(handler: zll::handler::AddressAssignment) -> Self {
        Self::Zll(handler.into())
    }
}

impl From<zll::handler::NetworkFound> for Handler {
    fn from(handler: zll::handler::NetworkFound) -> Self {
        Self::Zll(handler.into())
    }
}

impl From<zll::handler::ScanComplete> for Handler {
    fn from(handler: zll::handler::ScanComplete) -> Self {
        Self::Zll(handler.into())
    }
}

impl From<zll::handler::TouchLinkTarget> for Handler {
    fn from(handler: zll::handler::TouchLinkTarget) -> Self {
        Self::Zll(handler.into())
    }
}
