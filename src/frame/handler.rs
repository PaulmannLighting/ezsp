use crate::error::Decode;
use crate::frame::parameters::{
    binding, bootloader, cbke, green_power, messaging, mfglib, networking, security, trust_center,
    utilities, zll,
};
use crate::frame::Parameter;
use le_stream::FromLeStream;

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    Binding(binding::handler::Handler),
    Bootloader(bootloader::handler::Handler),
    CertificateBasedKeyExchange(cbke::handler::Handler),
    GreenPower(green_power::handler::Handler),
    Messaging(messaging::handler::Handler),
    MfgLib(mfglib::handler::Handler),
    Networking(networking::handler::Handler),
    Security(security::handler::Handler),
    TrustCenter(trust_center::handler::Handler),
    Utilities(utilities::handler::Handler),
    Zll(zll::handler::Handler),
}

impl Handler {
    #[allow(clippy::too_many_lines)]
    pub fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            // Binding callbacks.
            binding::handler::remote_delete_binding::Handler::ID => Ok(
                binding::handler::remote_delete_binding::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            binding::handler::remote_set_binding::Handler::ID => Ok(
                binding::handler::remote_set_binding::Handler::from_le_stream_exact(stream)?.into(),
            ),
            // Bootloader callbacks.
            bootloader::handler::bootload_transmit_complete::Handler::ID => Ok(
                bootloader::handler::bootload_transmit_complete::Handler::from_le_stream_exact(
                    stream,
                )?
                .into(),
            ),
            bootloader::handler::incoming_bootload_message::Handler::ID => Ok(
                bootloader::handler::incoming_bootload_message::Handler::from_le_stream_exact(
                    stream,
                )?
                .into(),
            ),
            // Certificate-based key exchange callbacks.
            cbke::handler::calculate_smacs::Handler::ID => {
                Ok(cbke::handler::calculate_smacs::Handler::from_le_stream_exact(stream)?.into())
            }
            cbke::handler::calculate_smacs283k1::Handler::ID => Ok(
                cbke::handler::calculate_smacs283k1::Handler::from_le_stream_exact(stream)?.into(),
            ),
            cbke::handler::dsa_sign::Handler::ID => {
                Ok(cbke::handler::dsa_sign::Handler::from_le_stream_exact(stream)?.into())
            }
            cbke::handler::dsa_verify::Handler::ID => {
                Ok(cbke::handler::dsa_verify::Handler::from_le_stream_exact(stream)?.into())
            }
            cbke::handler::generate_cbke_keys::Handler::ID => Ok(
                cbke::handler::generate_cbke_keys::Handler::from_le_stream_exact(stream)?.into(),
            ),
            cbke::handler::generate_cbke_keys283k1::Handler::ID => Ok(
                cbke::handler::generate_cbke_keys283k1::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            // Green Power callbacks.
            green_power::handler::incoming_message::Handler::ID => Ok(
                green_power::handler::incoming_message::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            green_power::handler::sent::Handler::ID => {
                Ok(green_power::handler::sent::Handler::from_le_stream_exact(stream)?.into())
            }
            // Messaging callbacks.
            messaging::handler::id_conflict::Handler::ID => {
                Ok(messaging::handler::id_conflict::Handler::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::incoming_many_to_one_route_request::Handler::ID => Ok(
                messaging::handler::incoming_many_to_one_route_request::Handler::from_le_stream_exact(
                    stream,
                )?
                .into(),
            ),
            messaging::handler::incoming_message::Handler::ID => Ok(
                messaging::handler::incoming_message::Handler::from_le_stream_exact(stream)?.into(),
            ),
            messaging::handler::incoming_network_status::Handler::ID => Ok(
                messaging::handler::incoming_network_status::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            messaging::handler::incoming_route_error::Handler::ID => Ok(
                messaging::handler::incoming_route_error::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            messaging::handler::incoming_sender_eui64::Handler::ID => Ok(
                messaging::handler::incoming_sender_eui64::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            messaging::handler::mac_filter_match_message::Handler::ID => Ok(
                messaging::handler::mac_filter_match_message::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            messaging::handler::mac_passthrough_message::Handler::ID => Ok(
                messaging::handler::mac_passthrough_message::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            messaging::handler::message_sent::Handler::ID => Ok(
                messaging::handler::message_sent::Handler::from_le_stream_exact(stream)?.into(),
            ),
            messaging::handler::poll::Handler::ID => {
                Ok(messaging::handler::poll::Handler::from_le_stream_exact(stream)?.into())
            }
            messaging::handler::poll_complete::Handler::ID => Ok(
                messaging::handler::poll_complete::Handler::from_le_stream_exact(stream)?.into(),
            ),
            messaging::handler::raw_transmit_complete::Handler::ID => Ok(
                messaging::handler::raw_transmit_complete::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            // MfgLib callbacks.
            mfglib::handler::rx::Handler::ID => {
                Ok(mfglib::handler::rx::Handler::from_le_stream_exact(stream)?.into())
            }
            // Networking callbacks.
            networking::handler::child_join::Handler::ID => Ok(
                networking::handler::child_join::Handler::from_le_stream_exact(stream)?.into(),
            ),
            networking::handler::duty_cycle::Handler::ID => Ok(
                networking::handler::duty_cycle::Handler::from_le_stream_exact(stream)?.into(),
            ),
            networking::handler::energy_scan_result::Handler::ID => Ok(
                networking::handler::energy_scan_result::Handler::from_le_stream_exact(stream)?.into(),
            ),
            networking::handler::network_found::Handler::ID => Ok(
                networking::handler::network_found::Handler::from_le_stream_exact(stream)?.into(),
            ),
            networking::handler::scan_complete::Handler::ID => Ok(
                networking::handler::scan_complete::Handler::from_le_stream_exact(stream)?.into(),
            ),
            networking::handler::stack_status::Handler::ID => Ok(
                networking::handler::stack_status::Handler::from_le_stream_exact(stream)?.into(),
            ),
            networking::handler::unused_pan_id_found::Handler::ID => Ok(
                networking::handler::unused_pan_id_found::Handler::from_le_stream_exact(stream)?.into(),
            ),
            // Security callbacks.
            security::handler::switch_network_key::Handler::ID => Ok(
                security::handler::switch_network_key::Handler::from_le_stream_exact(stream)?.into(),
            ),
            security::handler::zigbee_key_establishment::Handler::ID => Ok(
                security::handler::zigbee_key_establishment::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            // Trust Center callbacks.
            trust_center::handler::trust_center_join::Handler::ID => Ok(
                trust_center::handler::trust_center_join::Handler::from_le_stream_exact(stream)?
                    .into(),
            ),
            // Utilities callbacks.
            utilities::handler::counter_rollover::Handler::ID => Ok(
                utilities::handler::counter_rollover::Handler::from_le_stream_exact(stream)?.into(),
            ),
            utilities::handler::custom_frame::Handler::ID => Ok(
                utilities::handler::custom_frame::Handler::from_le_stream_exact(stream)?.into(),
            ),
            utilities::handler::stack_token_changed::Handler::ID => Ok(
                utilities::handler::stack_token_changed::Handler::from_le_stream_exact(stream)?.into(),
            ),
            utilities::handler::timer::Handler::ID => {
                Ok(utilities::handler::timer::Handler::from_le_stream_exact(stream)?.into())
            }
            // ZLL callbacks.
            zll::handler::address_assignment::Handler::ID => Ok(
                zll::handler::address_assignment::Handler::from_le_stream_exact(stream)?.into(),
            ),
            zll::handler::network_found::Handler::ID => Ok(
                zll::handler::network_found::Handler::from_le_stream_exact(stream)?.into(),
            ),
            zll::handler::scan_complete::Handler::ID => Ok(
                zll::handler::scan_complete::Handler::from_le_stream_exact(stream)?.into(),
            ),
            zll::handler::touch_link_target::Handler::ID => Ok(
                zll::handler::touch_link_target::Handler::from_le_stream_exact(stream)?.into(),
            ),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}

impl From<binding::handler::remote_delete_binding::Handler> for Handler {
    fn from(handler: binding::handler::remote_delete_binding::Handler) -> Self {
        Self::Binding(binding::handler::Handler::RemoteDeleteBinding(handler))
    }
}

impl From<binding::handler::remote_set_binding::Handler> for Handler {
    fn from(handler: binding::handler::remote_set_binding::Handler) -> Self {
        Self::Binding(binding::handler::Handler::RemoteSetBinding(handler))
    }
}

impl From<bootloader::handler::bootload_transmit_complete::Handler> for Handler {
    fn from(handler: bootloader::handler::bootload_transmit_complete::Handler) -> Self {
        Self::Bootloader(bootloader::handler::Handler::BootloadTransmitComplete(
            handler,
        ))
    }
}

impl From<bootloader::handler::incoming_bootload_message::Handler> for Handler {
    fn from(handler: bootloader::handler::incoming_bootload_message::Handler) -> Self {
        Self::Bootloader(bootloader::handler::Handler::IncomingBootloadMessage(
            handler,
        ))
    }
}

impl From<cbke::handler::calculate_smacs::Handler> for Handler {
    fn from(handler: cbke::handler::calculate_smacs::Handler) -> Self {
        Self::CertificateBasedKeyExchange(cbke::handler::Handler::CalculateSmacs(handler))
    }
}

impl From<cbke::handler::calculate_smacs283k1::Handler> for Handler {
    fn from(handler: cbke::handler::calculate_smacs283k1::Handler) -> Self {
        Self::CertificateBasedKeyExchange(cbke::handler::Handler::CalculateSmacs283k1(handler))
    }
}

impl From<cbke::handler::dsa_sign::Handler> for Handler {
    fn from(handler: cbke::handler::dsa_sign::Handler) -> Self {
        Self::CertificateBasedKeyExchange(cbke::handler::Handler::DsaSign(handler))
    }
}

impl From<cbke::handler::dsa_verify::Handler> for Handler {
    fn from(handler: cbke::handler::dsa_verify::Handler) -> Self {
        Self::CertificateBasedKeyExchange(cbke::handler::Handler::DsaVerify(handler))
    }
}

impl From<cbke::handler::generate_cbke_keys::Handler> for Handler {
    fn from(handler: cbke::handler::generate_cbke_keys::Handler) -> Self {
        Self::CertificateBasedKeyExchange(cbke::handler::Handler::GenerateCbkeKeys(handler))
    }
}

impl From<cbke::handler::generate_cbke_keys283k1::Handler> for Handler {
    fn from(handler: cbke::handler::generate_cbke_keys283k1::Handler) -> Self {
        Self::CertificateBasedKeyExchange(cbke::handler::Handler::GenerateCbkeKeys283k1(handler))
    }
}

impl From<green_power::handler::incoming_message::Handler> for Handler {
    fn from(handler: green_power::handler::incoming_message::Handler) -> Self {
        Self::GreenPower(green_power::handler::Handler::IncomingMessage(handler))
    }
}

impl From<green_power::handler::sent::Handler> for Handler {
    fn from(handler: green_power::handler::sent::Handler) -> Self {
        Self::GreenPower(green_power::handler::Handler::Sent(handler))
    }
}

impl From<messaging::handler::id_conflict::Handler> for Handler {
    fn from(handler: messaging::handler::id_conflict::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::IdConflict(handler))
    }
}

impl From<messaging::handler::incoming_many_to_one_route_request::Handler> for Handler {
    fn from(handler: messaging::handler::incoming_many_to_one_route_request::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::IncomingManyToOneRouteRequest(
            handler,
        ))
    }
}

impl From<messaging::handler::incoming_message::Handler> for Handler {
    fn from(handler: messaging::handler::incoming_message::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::IncomingMessage(handler))
    }
}

impl From<messaging::handler::incoming_network_status::Handler> for Handler {
    fn from(handler: messaging::handler::incoming_network_status::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::IncomingNetworkStatus(handler))
    }
}

impl From<messaging::handler::incoming_route_error::Handler> for Handler {
    fn from(handler: messaging::handler::incoming_route_error::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::IncomingRouteError(handler))
    }
}

impl From<messaging::handler::incoming_sender_eui64::Handler> for Handler {
    fn from(handler: messaging::handler::incoming_sender_eui64::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::IncomingSenderEui64(handler))
    }
}

impl From<messaging::handler::mac_filter_match_message::Handler> for Handler {
    fn from(handler: messaging::handler::mac_filter_match_message::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::MacFilterMatchMessage(handler))
    }
}

impl From<messaging::handler::mac_passthrough_message::Handler> for Handler {
    fn from(handler: messaging::handler::mac_passthrough_message::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::MacPassthroughMessage(handler))
    }
}

impl From<messaging::handler::message_sent::Handler> for Handler {
    fn from(handler: messaging::handler::message_sent::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::MessageSent(handler))
    }
}

impl From<messaging::handler::poll::Handler> for Handler {
    fn from(handler: messaging::handler::poll::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::Poll(handler))
    }
}

impl From<messaging::handler::poll_complete::Handler> for Handler {
    fn from(handler: messaging::handler::poll_complete::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::PollComplete(handler))
    }
}

impl From<messaging::handler::raw_transmit_complete::Handler> for Handler {
    fn from(handler: messaging::handler::raw_transmit_complete::Handler) -> Self {
        Self::Messaging(messaging::handler::Handler::RawTransmitComplete(handler))
    }
}

impl From<mfglib::handler::rx::Handler> for Handler {
    fn from(handler: mfglib::handler::rx::Handler) -> Self {
        Self::MfgLib(mfglib::handler::Handler::Rx(handler))
    }
}

impl From<networking::handler::child_join::Handler> for Handler {
    fn from(handler: networking::handler::child_join::Handler) -> Self {
        Self::Networking(networking::handler::Handler::ChildJoin(handler))
    }
}

impl From<networking::handler::duty_cycle::Handler> for Handler {
    fn from(handler: networking::handler::duty_cycle::Handler) -> Self {
        Self::Networking(networking::handler::Handler::DutyCycle(handler))
    }
}

impl From<networking::handler::energy_scan_result::Handler> for Handler {
    fn from(handler: networking::handler::energy_scan_result::Handler) -> Self {
        Self::Networking(networking::handler::Handler::EnergyScan(handler))
    }
}

impl From<networking::handler::network_found::Handler> for Handler {
    fn from(handler: networking::handler::network_found::Handler) -> Self {
        Self::Networking(networking::handler::Handler::NetworkFound(handler))
    }
}

impl From<networking::handler::scan_complete::Handler> for Handler {
    fn from(handler: networking::handler::scan_complete::Handler) -> Self {
        Self::Networking(networking::handler::Handler::ScanComplete(handler))
    }
}

impl From<networking::handler::stack_status::Handler> for Handler {
    fn from(handler: networking::handler::stack_status::Handler) -> Self {
        Self::Networking(networking::handler::Handler::StackStatus(handler))
    }
}

impl From<networking::handler::unused_pan_id_found::Handler> for Handler {
    fn from(handler: networking::handler::unused_pan_id_found::Handler) -> Self {
        Self::Networking(networking::handler::Handler::UnusedPanIdFound(handler))
    }
}

impl From<security::handler::switch_network_key::Handler> for Handler {
    fn from(handler: security::handler::switch_network_key::Handler) -> Self {
        Self::Security(security::handler::Handler::SwitchNetworkKey(handler))
    }
}

impl From<security::handler::zigbee_key_establishment::Handler> for Handler {
    fn from(handler: security::handler::zigbee_key_establishment::Handler) -> Self {
        Self::Security(security::handler::Handler::ZigbeeKeyEstablishment(handler))
    }
}

impl From<trust_center::handler::trust_center_join::Handler> for Handler {
    fn from(handler: trust_center::handler::trust_center_join::Handler) -> Self {
        Self::TrustCenter(trust_center::handler::Handler::TrustCenterJoin(handler))
    }
}

impl From<utilities::handler::counter_rollover::Handler> for Handler {
    fn from(handler: utilities::handler::counter_rollover::Handler) -> Self {
        Self::Utilities(utilities::handler::Handler::CounterRollover(handler))
    }
}

impl From<utilities::handler::custom_frame::Handler> for Handler {
    fn from(handler: utilities::handler::custom_frame::Handler) -> Self {
        Self::Utilities(utilities::handler::Handler::CustomFrame(handler))
    }
}

impl From<utilities::handler::stack_token_changed::Handler> for Handler {
    fn from(handler: utilities::handler::stack_token_changed::Handler) -> Self {
        Self::Utilities(utilities::handler::Handler::StackTokenChanged(handler))
    }
}

impl From<utilities::handler::timer::Handler> for Handler {
    fn from(handler: utilities::handler::timer::Handler) -> Self {
        Self::Utilities(utilities::handler::Handler::Timer(handler))
    }
}

impl From<zll::handler::address_assignment::Handler> for Handler {
    fn from(handler: zll::handler::address_assignment::Handler) -> Self {
        Self::Zll(zll::handler::Handler::AddressAssignment(handler))
    }
}

impl From<zll::handler::network_found::Handler> for Handler {
    fn from(handler: zll::handler::network_found::Handler) -> Self {
        Self::Zll(zll::handler::Handler::NetworkFound(handler))
    }
}

impl From<zll::handler::scan_complete::Handler> for Handler {
    fn from(handler: zll::handler::scan_complete::Handler) -> Self {
        Self::Zll(zll::handler::Handler::ScanComplete(handler))
    }
}

impl From<zll::handler::touch_link_target::Handler> for Handler {
    fn from(handler: zll::handler::touch_link_target::Handler) -> Self {
        Self::Zll(zll::handler::Handler::TouchLinkTarget(handler))
    }
}
