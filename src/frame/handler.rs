use crate::error::Decode;
use crate::frame::parameters::{
    binding, bootloader, cbke, green_power, messaging, mfglib, networking, security, trust_center,
    utilities, zll,
};
use crate::frame::Parameter;
use crate::parsable::Parsable;
use le_stream::FromLeStream;

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

impl Parsable for Handler {
    /// Parse a handler from a little-endian stream.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame ID is not recognized.
    #[allow(clippy::too_many_lines)]
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match Some(id) {
            // Binding callbacks.
            binding::handler::RemoteDeleteBinding::ID => {
                Ok(binding::handler::RemoteDeleteBinding::from_le_stream_exact(stream)?.into())
            }
            binding::handler::RemoteSetBinding::ID => {
                Ok(binding::handler::RemoteSetBinding::from_le_stream_exact(stream)?.into())
            }
            // Bootloader callbacks.
            bootloader::handler::BootloadTransmitComplete::ID => Ok(
                bootloader::handler::BootloadTransmitComplete::from_le_stream_exact(stream)?.into(),
            ),
            bootloader::handler::IncomingBootloadMessage::ID => Ok(
                bootloader::handler::IncomingBootloadMessage::from_le_stream_exact(stream)?.into(),
            ),
            // Certificate-based key exchange callbacks.
            cbke::handler::CalculateSmacs::ID => {
                Ok(cbke::handler::CalculateSmacs::from_le_stream_exact(stream)?.into())
            }
            cbke::handler::CalculateSmacs283k1::ID => {
                Ok(cbke::handler::CalculateSmacs283k1::from_le_stream_exact(stream)?.into())
            }
            cbke::handler::DsaSign::ID => {
                Ok(cbke::handler::DsaSign::from_le_stream_exact(stream)?.into())
            }
            cbke::handler::DsaVerify::ID => {
                Ok(cbke::handler::DsaVerify::from_le_stream_exact(stream)?.into())
            }
            cbke::handler::GenerateCbkeKeys::ID => {
                Ok(cbke::handler::GenerateCbkeKeys::from_le_stream_exact(stream)?.into())
            }
            cbke::handler::GenerateCbkeKeys283k1::ID => {
                Ok(cbke::handler::GenerateCbkeKeys283k1::from_le_stream_exact(stream)?.into())
            }
            // Green Power callbacks.
            green_power::handler::IncomingMessage::ID => {
                Ok(green_power::handler::IncomingMessage::from_le_stream_exact(stream)?.into())
            }
            green_power::handler::Sent::ID => {
                Ok(green_power::handler::Sent::from_le_stream_exact(stream)?.into())
            }
            // Messaging callbacks.
            messaging::handler::IdConflict::ID => {
                Ok(messaging::handler::IdConflict::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::IncomingManyToOneRouteRequest::ID => Ok(
                messaging::handler::IncomingManyToOneRouteRequest::from_le_stream_exact(stream)?
                    .into(),
            ),
            messaging::handler::IncomingMessage::ID => {
                Ok(messaging::handler::IncomingMessage::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::IncomingNetworkStatus::ID => {
                Ok(messaging::handler::IncomingNetworkStatus::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::IncomingRouteError::ID => {
                Ok(messaging::handler::IncomingRouteError::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::IncomingSenderEui64::ID => {
                Ok(messaging::handler::IncomingSenderEui64::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::MacFilterMatchMessage::ID => {
                Ok(messaging::handler::MacFilterMatchMessage::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::MacPassthroughMessage::ID => {
                Ok(messaging::handler::MacPassthroughMessage::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::MessageSent::ID => {
                Ok(messaging::handler::MessageSent::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::Poll::ID => {
                Ok(messaging::handler::Poll::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::PollComplete::ID => {
                Ok(messaging::handler::PollComplete::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::RawTransmitComplete::ID => {
                Ok(messaging::handler::RawTransmitComplete::from_le_stream_exact(stream)?.into())
            }
            // MfgLib callbacks.
            mfglib::handler::Rx::ID => {
                Ok(mfglib::handler::Rx::from_le_stream_exact(stream)?.into())
            }
            // Networking callbacks.
            networking::handler::ChildJoin::ID => {
                Ok(networking::handler::ChildJoin::from_le_stream_exact(stream)?.into())
            }
            networking::handler::DutyCycle::ID => {
                Ok(networking::handler::DutyCycle::from_le_stream_exact(stream)?.into())
            }
            networking::handler::EnergyScanResult::ID => {
                Ok(networking::handler::EnergyScanResult::from_le_stream_exact(stream)?.into())
            }
            networking::handler::NetworkFound::ID => {
                Ok(networking::handler::NetworkFound::from_le_stream_exact(stream)?.into())
            }
            networking::handler::ScanComplete::ID => {
                Ok(networking::handler::ScanComplete::from_le_stream_exact(stream)?.into())
            }
            networking::handler::StackStatus::ID => {
                Ok(networking::handler::StackStatus::from_le_stream_exact(stream)?.into())
            }
            networking::handler::UnusedPanIdFound::ID => {
                Ok(networking::handler::UnusedPanIdFound::from_le_stream_exact(stream)?.into())
            }
            // Security callbacks.
            security::handler::SwitchNetworkKey::ID => {
                Ok(security::handler::SwitchNetworkKey::from_le_stream_exact(stream)?.into())
            }
            security::handler::ZigbeeKeyEstablishment::ID => {
                Ok(security::handler::ZigbeeKeyEstablishment::from_le_stream_exact(stream)?.into())
            }
            // Trust Center callbacks.
            trust_center::handler::TrustCenterJoin::ID => {
                Ok(trust_center::handler::TrustCenterJoin::from_le_stream_exact(stream)?.into())
            }
            // Utilities callbacks.
            utilities::handler::CounterRollover::ID => {
                Ok(utilities::handler::CounterRollover::from_le_stream_exact(stream)?.into())
            }
            utilities::handler::CustomFrame::ID => {
                Ok(utilities::handler::CustomFrame::from_le_stream_exact(stream)?.into())
            }
            utilities::handler::StackTokenChanged::ID => {
                Ok(utilities::handler::StackTokenChanged::from_le_stream_exact(stream)?.into())
            }
            utilities::handler::Timer::ID => {
                Ok(utilities::handler::Timer::from_le_stream_exact(stream)?.into())
            }
            // ZLL callbacks.
            zll::handler::AddressAssignment::ID => {
                Ok(zll::handler::AddressAssignment::from_le_stream_exact(stream)?.into())
            }
            zll::handler::NetworkFound::ID => {
                Ok(zll::handler::NetworkFound::from_le_stream_exact(stream)?.into())
            }
            zll::handler::ScanComplete::ID => {
                Ok(zll::handler::ScanComplete::from_le_stream_exact(stream)?.into())
            }
            zll::handler::TouchLinkTarget::ID => {
                Ok(zll::handler::TouchLinkTarget::from_le_stream_exact(stream)?.into())
            }
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
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
