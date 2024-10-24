use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;
use crate::parameters::{
    binding, bootloader, cbke, configuration, green_power, messaging, mfglib, networking, security,
    token_interface, trust_center, utilities, wwah, zll,
};

const VERSION_ID: u16 = <configuration::version::Response as Parameter>::ID as u16;

/// `EZSP` response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response parameters for binding frames.
    Binding(binding::Response),
    /// Response parameters for bootloader frames.
    Bootloader(bootloader::Response),
    /// Response parameters for certificate-based key exchange (CBKE) frames.
    Cbke(cbke::Response),
    /// Response parameters for configuration frames.
    Configuration(configuration::Response),
    /// Response parameters for green power frames.
    GreenPower(green_power::Response),
    /// Response parameters for messaging frames.
    Messaging(messaging::Response),
    /// Response parameters for Manufacturing Test Library (`MfgLib`) frames.
    MfgLib(mfglib::Response),
    /// Response parameters for networking frames.
    Networking(networking::Response),
    /// Response parameters for security frames.
    Security(security::Response),
    /// Response parameters for token interface frames.
    TokenInterface(token_interface::Response),
    /// Response parameters for trust center frames.
    TrustCenter(trust_center::Response),
    /// Response parameters for utilities frames.
    Utilities(utilities::Response),
    /// Response parameters for Wireless Workgroup for Home Automation (WWAH) frames.
    Wwah(wwah::Response),
    /// Response parameters for Zigbee Light Link (ZLL) frames.
    Zll(zll::Response),
}

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            // Binding responses
            <binding::binding_is_active::Response as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::BindingIsActive(
                    binding::binding_is_active::Response::from_le_stream_exact(stream)?,
                )))
            }
            <binding::clear_binding_table::Response as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::ClearBindingTable(
                    binding::clear_binding_table::Response::from_le_stream_exact(stream)?,
                )))
            }
            <binding::delete_binding::Response as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::DeleteBinding(
                    binding::delete_binding::Response::from_le_stream_exact(stream)?,
                )))
            }
            <binding::get_binding::Response as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::GetBinding(
                    binding::get_binding::Response::from_le_stream_exact(stream)?,
                )))
            }
            <binding::get_binding_remote_node_id::Response as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::GetBindingRemoteNodeId(
                    binding::get_binding_remote_node_id::Response::from_le_stream_exact(stream)?,
                )))
            }
            <binding::set_binding::Response as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::SetBinding(
                    binding::set_binding::Response::from_le_stream_exact(stream)?,
                )))
            }
            <binding::set_binding_remote_node_id::Response as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::SetBindingRemoteNodeId(
                    binding::set_binding_remote_node_id::Response::from_le_stream_exact(stream)?,
                )))
            }
            <binding::handler::RemoteDeleteBinding as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::Handler(binding::handler::Handler::RemoteDeleteBinding(
                    binding::handler::RemoteDeleteBinding::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::handler::RemoteSetBinding as Parameter>::ID => {
                Ok(Self::Binding(binding::Response::Handler(binding::handler::Handler::RemoteSetBinding(
                    binding::handler::RemoteSetBinding::from_le_stream_exact(stream)?,
                ))))
            }
            // Bootloader responses
            <bootloader::aes_encrypt::Response as Parameter>::ID => {
                Ok(Self::Bootloader(bootloader::Response::AesEncrypt(
                    bootloader::aes_encrypt::Response::from_le_stream_exact(stream)?,
                )))
            }
            <bootloader::get_standalone_bootloader_version_plat_micro_phy::Response as Parameter>::ID => {
                Ok(Self::Bootloader(bootloader::Response::GetStandaloneBootloaderVersionPlatMicroPhy(
                    bootloader::get_standalone_bootloader_version_plat_micro_phy::Response::from_le_stream_exact(stream)?,
                )))
            }
            <bootloader::launch_standalone_bootloader::Response as Parameter>::ID => {
                Ok(Self::Bootloader(bootloader::Response::LaunchStandaloneBootloader(
                    bootloader::launch_standalone_bootloader::Response::from_le_stream_exact(stream)?,
                )))
            }
            // FIXME: ID collision with networking::set_radio_ieee802154_cca_mode. Was this removed?
            /*
            <bootloader::override_current_channel::Response as Parameter>::ID => {
                Ok(Self::Bootloader(bootloader::Response::OverrideCurrentChannel(
                    bootloader::override_current_channel::Response::from_le_stream_exact(stream)?,
                )))
            }*/
            <bootloader::send_bootload_message::Response as Parameter>::ID => {
                Ok(Self::Bootloader(bootloader::Response::SendBootloadMessage(
                    bootloader::send_bootload_message::Response::from_le_stream_exact(stream)?,
                )))
            }
            <bootloader::handler::BootloadTransmitComplete as Parameter>::ID => {
                Ok(Self::Bootloader(bootloader::Response::Handler(bootloader::handler::Handler::BootloadTransmitComplete(
                    bootloader::handler::BootloadTransmitComplete::from_le_stream_exact(stream)?,
                ))))
            }
            <bootloader::handler::IncomingBootloadMessage as Parameter>::ID => {
                Ok(Self::Bootloader(bootloader::Response::Handler(bootloader::handler::Handler::IncomingBootloadMessage(
                    bootloader::handler::IncomingBootloadMessage::from_le_stream_exact(stream)?,
                ))))
            }
            // CBKE responses
            <cbke::dsa_sign::Response as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::DsaSign(
                    cbke::dsa_sign::Response::from_le_stream_exact(stream)?,
                )))
            }
            <cbke::dsa_verify::Response as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::DsaVerify(
                    cbke::dsa_verify::Response::from_le_stream_exact(stream)?,
                )))
            }
            <cbke::dsa_verify283k1::Response as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::DsaVerify283k1(
                    cbke::dsa_verify283k1::Response::from_le_stream_exact(stream)?,
                )))
            }
            <cbke::generate_cbke_keys::Response as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::GenerateCbkeKeys(
                    cbke::generate_cbke_keys::Response::from_le_stream_exact(stream)?,
                )))
            }
            <cbke::generate_cbke_keys283k1::Response as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::GenerateCbkeKeys283k1(
                    cbke::generate_cbke_keys283k1::Response::from_le_stream_exact(stream)?,
                )))
            }
            <cbke::get_certificate::Response as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::GetCertificate(
                    cbke::get_certificate::Response::from_le_stream_exact(stream)?,
                )))
            }
            <cbke::save_preinstalled_cbke_data283k1::Response as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::SavePreinstalledCbkeData283k1(
                    cbke::save_preinstalled_cbke_data283k1::Response::from_le_stream_exact(stream)?,
                )))
            }
            <cbke::set_preinstalled_cbke_data::Response as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::SetPreinstalledCbkeData(
                    cbke::set_preinstalled_cbke_data::Response::from_le_stream_exact(stream)?,
                )))
            }
            <cbke::handler::CalculateSmacs as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::Handler(cbke::handler::Handler::CalculateSmacs(
                    cbke::handler::CalculateSmacs::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::CalculateSmacs283k1 as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::Handler(cbke::handler::Handler::CalculateSmacs283k1(
                    cbke::handler::CalculateSmacs283k1::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::DsaSign as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::Handler(cbke::handler::Handler::DsaSign(
                    cbke::handler::DsaSign::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::DsaVerify as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::Handler(cbke::handler::Handler::DsaVerify(
                    cbke::handler::DsaVerify::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::GenerateCbkeKeys as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::Handler(cbke::handler::Handler::GenerateCbkeKeys283k1(
                    cbke::handler::GenerateCbkeKeys283k1::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::GenerateCbkeKeys283k1 as Parameter>::ID => {
                Ok(Self::Cbke(cbke::Response::Handler(cbke::handler::Handler::GenerateCbkeKeys283k1(
                    cbke::handler::GenerateCbkeKeys283k1::from_le_stream_exact(stream)?,
                ))))
            }
            // Configuration responses
            <configuration::add_endpoint::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::AddEndpoint(
                    configuration::add_endpoint::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::get_configuration_value::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::GetConfigurationValue(
                    configuration::get_configuration_value::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::get_extended_value::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::GetExtendedValue(
                    configuration::get_extended_value::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::get_policy::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::GetPolicy(
                    configuration::get_policy::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::get_value::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::GetValue(
                    configuration::get_value::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::read_attribute::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::ReadAttribute(
                    configuration::read_attribute::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::send_pan_id_update::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::SendPanIdUpdate(
                    configuration::send_pan_id_update::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::set_configuration_value::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::SetConfigurationValue(
                    configuration::set_configuration_value::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::set_passive_ack_config::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::SetPassiveAckConfig(
                    configuration::set_passive_ack_config::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::set_policy::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::SetPolicy(
                    configuration::set_policy::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::set_value::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::SetValue(
                    configuration::set_value::Response::from_le_stream_exact(stream)?,
                )))
            }
            VERSION_ID => {
                Ok(Self::Configuration(configuration::Response::Version(
                    configuration::version::Response::from_le_stream_exact(stream)?,
                )))
            }
            <configuration::write_attribute::Response as Parameter>::ID => {
                Ok(Self::Configuration(configuration::Response::WriteAttribute(
                    configuration::write_attribute::Response::from_le_stream_exact(stream)?,
                )))
            }
            // Green Power responses
            <green_power::proxy_table::get_entry::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::ProxyTable(green_power::proxy_table::Response::GetEntry(
                    green_power::proxy_table::get_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::proxy_table::lookup::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::ProxyTable(green_power::proxy_table::Response::Lookup(
                    green_power::proxy_table::lookup::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::proxy_table::process_gp_pairing::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::ProxyTable(green_power::proxy_table::Response::ProcessGpPairing(
                    green_power::proxy_table::process_gp_pairing::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::send::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::Send(
                    green_power::send::Response::from_le_stream_exact(stream)?,
                )))
            }
            <green_power::sink_commission::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkCommission(
                    green_power::sink_commission::Response::from_le_stream_exact(stream)?,
                )))
            }
            <green_power::sink_table::clear_all::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::ClearAll(
                    green_power::sink_table::clear_all::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_table::find_or_allocate_entry::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::FindOrAllocateEntry(
                    green_power::sink_table::find_or_allocate_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_table::get_entry::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::GetEntry(
                    green_power::sink_table::get_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_table::init::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::Init(
                    green_power::sink_table::init::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_table::lookup::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::Lookup(
                    green_power::sink_table::lookup::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_table::number_of_active_entries::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::NumberOfActiveEntries(
                    green_power::sink_table::number_of_active_entries::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_table::remove_entry::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::RemoveEntry(
                    green_power::sink_table::remove_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_table::set_entry::Response as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::SetEntry(
                    green_power::sink_table::set_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::handler::IncomingMessage as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::Handler(green_power::handler::Handler::IncomingMessage(
                    green_power::handler::IncomingMessage::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::handler::Sent as Parameter>::ID => {
                Ok(Self::GreenPower(green_power::Response::Handler(green_power::handler::Handler::Sent(
                    green_power::handler::Sent::from_le_stream_exact(stream)?,
                ))))
            }
            // Messaging responses
            <messaging::address_table_entry_is_active::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::AddressTableEntryIsActive(
                    messaging::address_table_entry_is_active::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::get_address_table_remote_eui64::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::GetAddressTableRemoteEui64(
                    messaging::get_address_table_remote_eui64::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::get_address_table_remote_node_id::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::GetAddressTableRemoteNodeId(
                    messaging::get_address_table_remote_node_id::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::get_beacon_classification_params::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::GetBeaconClassificationParams(
                    messaging::get_beacon_classification_params::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::get_extended_timeout::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::GetExtendedTimeout(
                    messaging::get_extended_timeout::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::get_multicast_table_entry::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::GetMulticastTableEntry(
                    messaging::get_multicast_table_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::lookup_eui64_by_node_id::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::LookupEui64ByNodeId(
                    messaging::lookup_eui64_by_node_id::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::lookup_node_id_by_eui64::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::LookupNodeIdByEui64(
                    messaging::lookup_node_id_by_eui64::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::maximum_payload_length::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::MaximumPayloadLength(
                    messaging::maximum_payload_length::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::poll_for_data::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::PollForData(
                    messaging::poll_for_data::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::proxy_broadcast::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::ProxyBroadcast(
                    messaging::proxy_broadcast::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::replace_address_table_entry::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::ReplaceAddressTableEntry(
                    messaging::replace_address_table_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::send_broadcast::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SendBroadcast(
                    messaging::send_broadcast::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::send_many_to_one_route_request::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SendManyToOneRouteRequest(
                    messaging::send_many_to_one_route_request::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::send_multicast::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SendMulticast(
                    messaging::send_multicast::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::send_multicast_with_alias::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SendMulticastWithAlias(
                    messaging::send_multicast_with_alias::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::send_raw_message::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SendRawMessage(
                    messaging::send_raw_message::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::send_raw_message_extended::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SendRawMessageExtended(
                    messaging::send_raw_message_extended::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::send_reply::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SendReply(
                    messaging::send_reply::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::send_unicast::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SendUnicast(
                    messaging::send_unicast::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::set_address_table_remote_eui64::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SetAddressTableRemoteEui64(
                    messaging::set_address_table_remote_eui64::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::set_address_table_remote_node_id::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SetAddressTableRemoteNodeId(
                    messaging::set_address_table_remote_node_id::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::set_beacon_classification_params::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SetBeaconClassificationParams(
                    messaging::set_beacon_classification_params::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::set_extended_timeout::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SetExtendedTimeout(
                    messaging::set_extended_timeout::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::set_mac_poll_failure_wait_time::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SetMacPollFailureWaitTime(
                    messaging::set_mac_poll_failure_wait_time::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::set_multicast_table_entry::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SetMulticastTableEntry(
                    messaging::set_multicast_table_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::set_source_route_discovery_mode::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::SetSourceRouteDiscoveryMode(
                    messaging::set_source_route_discovery_mode::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::unicast_current_network_key::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::UnicastCurrentNetworkKey(
                    messaging::unicast_current_network_key::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::write_node_data::Response as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::WriteNodeData(
                    messaging::write_node_data::Response::from_le_stream_exact(stream)?,
                )))
            }
            <messaging::handler::IdConflict as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::IdConflict(
                    messaging::handler::IdConflict::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::IncomingManyToOneRouteRequest as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::IncomingManyToOneRouteRequest(
                    messaging::handler::IncomingManyToOneRouteRequest::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::IncomingMessage as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::IncomingMessage(
                    messaging::handler::IncomingMessage::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::IncomingNetworkStatus as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::IncomingNetworkStatus(
                    messaging::handler::IncomingNetworkStatus::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::IncomingRouteError as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::IncomingRouteError(
                    messaging::handler::IncomingRouteError::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::IncomingSenderEui64 as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::IncomingSenderEui64(
                    messaging::handler::IncomingSenderEui64::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::MacFilterMatchMessage as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::MacFilterMatchMessage(
                    messaging::handler::MacFilterMatchMessage::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::MacPassthroughMessage as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::MacPassthroughMessage(
                    messaging::handler::MacPassthroughMessage::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::MessageSent as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::MessageSent(
                    messaging::handler::MessageSent::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::Poll as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::Poll(
                    messaging::handler::Poll::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::PollComplete as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::PollComplete(
                    messaging::handler::PollComplete::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::RawTransmitComplete as Parameter>::ID => {
                Ok(Self::Messaging(messaging::Response::Handler(messaging::handler::Handler::RawTransmitComplete(
                    messaging::handler::RawTransmitComplete::from_le_stream_exact(stream)?,
                ))))
            }
            // MfgLib responses
            <mfglib::end::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::End(
                    mfglib::end::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::get_channel::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::GetChannel(
                    mfglib::get_channel::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::get_power::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::GetPower(
                    mfglib::get_power::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::send_packet::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::SendPacket(
                    mfglib::send_packet::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::set_channel::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::SetChannel(
                    mfglib::set_channel::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::set_power::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::SetPower(
                    mfglib::set_power::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::start::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::Start(
                    mfglib::start::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::start_stream::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::StartStream(
                    mfglib::start_stream::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::start_tone::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::StartTone(
                    mfglib::start_tone::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::stop_stream::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::StopStream(
                    mfglib::stop_stream::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::stop_tone::Response as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::StopTone(
                    mfglib::stop_tone::Response::from_le_stream_exact(stream)?,
                )))
            }
            <mfglib::handler::Rx as Parameter>::ID => {
                Ok(Self::MfgLib(mfglib::Response::Handler(mfglib::handler::Handler::Rx(
                    mfglib::handler::Rx::from_le_stream_exact(stream)?,
                ))))
            }
            // Networking responses
            <networking::child_id::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::ChildId(
                    networking::child_id::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::clear_stored_beacons::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::ClearStoredBeacons(
                    networking::clear_stored_beacons::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::energy_scan_request::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::EnergyScanRequest(
                    networking::energy_scan_request::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::find_and_rejoin_network::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::FindAndRejoinNetwork(
                    networking::find_and_rejoin_network::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::find_unused_pan_id::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::FindUnusedPanId(
                    networking::find_unused_pan_id::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::form_network::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::FormNetwork(
                    networking::form_network::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_child_data::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetChildData(
                    networking::get_child_data::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_current_duty_cycle::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetCurrentDutyCycle(
                    networking::get_current_duty_cycle::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_duty_cycle_limits::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetDutyCycleLimits(
                    networking::get_duty_cycle_limits::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_duty_cycle_state::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetDutyCycleState(
                    networking::get_duty_cycle_state::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_first_beacon::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetFirstBeacon(
                    networking::get_first_beacon::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_logical_channel::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetLogicalChannel(
                    networking::get_logical_channel::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_neighbor::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetNeighbor(
                    networking::get_neighbor::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_neighbor_frame_counter::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetNeighborFrameCounter(
                    networking::get_neighbor_frame_counter::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_network_parameters::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetNetworkParameters(
                    networking::get_network_parameters::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_next_beacon::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetNextBeacon(
                    networking::get_next_beacon::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_num_stored_beacons::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetNumStoredBeacons(
                    networking::get_num_stored_beacons::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_parent_child_parameters::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetParentChildParameters(
                    networking::get_parent_child_parameters::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_radio_channel::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetRadioChannel(
                    networking::get_radio_channel::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_radio_parameters::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetRadioParameters(
                    networking::get_radio_parameters::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_route_table_entry::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetRouteTableEntry(
                    networking::get_route_table_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_routing_shortcut_threshold::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetRoutingShortcutThreshold(
                    networking::get_routing_shortcut_threshold::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_source_route_table_entry::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetSourceRouteTableEntry(
                    networking::get_source_route_table_entry::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_source_route_table_filled_size::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetSourceRouteTableFilledSize(
                    networking::get_source_route_table_filled_size::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::get_source_route_table_total_size::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::GetSourceRouteTableTotalSize(
                    networking::get_source_route_table_total_size::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::id::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::Id(
                    networking::id::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::join_network::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::JoinNetwork(
                    networking::join_network::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::join_network_directly::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::JoinNetworkDirectly(
                    networking::join_network_directly::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::leave_network::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::LeaveNetwork(
                    networking::leave_network::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::multi_phy_set_radio_channel::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::MultiPhySetRadioChannel(
                    networking::multi_phy_set_radio_channel::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::multi_phy_set_radio_power::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::MultiPhySetRadioPower(
                    networking::multi_phy_set_radio_power::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::multi_phy_start::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::MultiPhyStart(
                    networking::multi_phy_start::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::multi_phy_stop::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::MultiPhyStop(
                    networking::multi_phy_stop::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::neighbor_count::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::NeighborCount(
                    networking::neighbor_count::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::network_init::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::NetworkInit(
                    networking::network_init::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::network_state::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::NetworkState(
                    networking::network_state::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::permit_joining::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::PermitJoining(
                    networking::permit_joining::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::send_link_power_delta_request::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SendLinkPowerDeltaRequest(
                    networking::send_link_power_delta_request::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_broken_route_error_code::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetBrokenRouteErrorCode(
                    networking::set_broken_route_error_code::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_child_data::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetChildData(
                    networking::set_child_data::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_concentrator::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetConcentrator(
                    networking::set_concentrator::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_duty_cycle_limits_in_stack::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetDutyCycleLimitsInStack(
                    networking::set_duty_cycle_limits_in_stack::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_logical_and_radio_channel::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetLogicalAndRadioChannel(
                    networking::set_logical_and_radio_channel::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_manufacturer_code::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetManufacturerCode(networking::set_manufacturer_code::Response::from_le_stream_exact(stream)?)))
            }
            <networking::set_neighbor_frame_counter::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetNeighborFrameCounter(
                    networking::set_neighbor_frame_counter::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_power_descriptor::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetPowerDescriptor(
                    networking::set_power_descriptor::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_radio_channel::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetRadioChannel(
                    networking::set_radio_channel::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_radio_ieee802154_cca_mode::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetRadioIeee802154CcaMode(
                    networking::set_radio_ieee802154_cca_mode::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_radio_power::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetRadioPower(
                    networking::set_radio_power::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::set_routing_shortcut_threshold::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::SetRoutingShortcutThreshold(
                    networking::set_routing_shortcut_threshold::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::start_scan::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::StartScan(
                    networking::start_scan::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::stop_scan::Response as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::StopScan(
                    networking::stop_scan::Response::from_le_stream_exact(stream)?,
                )))
            }
            <networking::handler::ChildJoin as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::Handler(networking::handler::Handler::ChildJoin(
                    networking::handler::ChildJoin::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::DutyCycle as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::Handler(networking::handler::Handler::DutyCycle(
                    networking::handler::DutyCycle::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::EnergyScanResult as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::Handler(networking::handler::Handler::EnergyScanResult(
                    networking::handler::EnergyScanResult::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::NetworkFound as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::Handler(networking::handler::Handler::NetworkFound(
                    networking::handler::NetworkFound::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::ScanComplete as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::Handler(networking::handler::Handler::ScanComplete(
                    networking::handler::ScanComplete::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::StackStatus as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::Handler(networking::handler::Handler::StackStatus(
                    networking::handler::StackStatus::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::UnusedPanIdFound as Parameter>::ID => {
                Ok(Self::Networking(networking::Response::Handler(networking::handler::Handler::UnusedPanIdFound(
                    networking::handler::UnusedPanIdFound::from_le_stream_exact(stream)?,
                ))))
            }
        }
    }
}
