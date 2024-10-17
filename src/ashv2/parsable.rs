use crate::error::Decode;
use crate::frame::Parameter;
use crate::parameters::{
    binding, bootloader, cbke, green_power, messaging, mfglib, networking, security, trust_center,
    utilities, zll,
};
use crate::Handler;
use le_stream::FromLeStream;

/// A trait for parsing parameters from a little-endian stream given their frame ID.
pub trait Parsable: Sized {
    /// Parse a parameter from a little-endian stream given its frame ID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the parsing of the parameter failed.
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>;
}

impl<T> Parsable for T
where
    T: Parameter + FromLeStream,
{
    fn parse_from_le_stream<S>(id: u16, stream: S) -> Result<Self, Decode>
    where
        S: Iterator<Item = u8>,
    {
        let Some(my_id) = Self::ID else {
            return Err(Decode::MissingId);
        };

        if my_id.into() != id {
            return Err(Decode::FrameIdMismatch {
                expected: my_id.into(),
                found: id,
            });
        }

        Ok(Self::from_le_stream_exact(stream)?)
    }
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
