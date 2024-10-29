use super::{Callback, Response};
use crate::error::Decode;
use crate::frame::parameters::{
    binding, bootloader, cbke, configuration, green_power, messaging, mfglib, networking,
};
use crate::frame::Identified;
use crate::parameters::{security, token_interface, trust_center, utilities, wwah, zll};
use crate::Parsable;
use le_stream::FromLeStream;

impl Parsable for super::Parameters {
    #[allow(clippy::too_many_lines)]
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        const VERSION_ID: u16 = configuration::version::Response::ID as u16;

        match id {
            // Binding responses
            <binding::clear_table::Response as Identified>::ID => {
                Ok(Self::Response(Response::Binding(binding::Response::ClearTable(
                    binding::clear_table::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::delete::Response as Identified>::ID => {
                Ok(Self::Response(Response::Binding(binding::Response::Delete(
                    binding::delete::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::get::Response as Identified>::ID => {
                Ok(Self::Response(Response::Binding(binding::Response::Get(
                    binding::get::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::get_remote_node_id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Binding(binding::Response::GetRemoteNodeId(
                    binding::get_remote_node_id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::is_active::Response as Identified>::ID => {
                Ok(Self::Response(Response::Binding(binding::Response::IsActive(
                    binding::is_active::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::set::Response as Identified>::ID => {
                Ok(Self::Response(Response::Binding(binding::Response::Set(
                    binding::set::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::set_remote_node_id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Binding(binding::Response::SetRemoteNodeId(
                    binding::set_remote_node_id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::handler::RemoteDeleteBinding as Identified>::ID => {
                Ok(Self::Handler(Callback::Binding(binding::handler::Handler::RemoteDeleteBinding(
                    binding::handler::RemoteDeleteBinding::from_le_stream_exact(stream)?,
                ))))
            }
            <binding::handler::RemoteSetBinding as Identified>::ID => {
                Ok(Self::Handler(Callback::Binding(binding::handler::Handler::RemoteSetBinding(
                    binding::handler::RemoteSetBinding::from_le_stream_exact(stream)?.into(),
                ))))
            }
            // Bootloader responses
            <bootloader::aes_encrypt::Response as Identified>::ID => {
                Ok(Self::Response(Response::Bootloader(bootloader::Response::AesEncrypt(
                    bootloader::aes_encrypt::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <bootloader::get_standalone_bootloader_version_plat_micro_phy::Response as Identified>::ID => {
                Ok(Self::Response(Response::Bootloader(bootloader::Response::GetStandaloneBootloaderVersionPlatMicroPhy(
                    bootloader::get_standalone_bootloader_version_plat_micro_phy::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <bootloader::launch_standalone_bootloader::Response as Identified>::ID => {
                Ok(Self::Response(Response::Bootloader(bootloader::Response::LaunchStandaloneBootloader(
                    bootloader::launch_standalone_bootloader::Response::from_le_stream_exact(stream)?,
                ))))
            }
            // FIXME: ID collision with networking::set_radio_ieee802154_cca_mode. Was this removed?
            /*
            <bootloader::override_current_channel::Response as Parameter>::ID => {
                Ok(Self::Response(Response::Bootloader(bootloader::Response::OverrideCurrentChannel(
                    bootloader::override_current_channel::Response::from_le_stream_exact(stream)?,
                ))))
            }*/
            <bootloader::send_bootload_message::Response as Identified>::ID => {
                Ok(Self::Response(Response::Bootloader(bootloader::Response::SendBootloadMessage(
                    bootloader::send_bootload_message::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <bootloader::handler::BootloadTransmitComplete as Identified>::ID => {
                Ok(Self::Handler(Callback::Bootloader(bootloader::handler::Handler::BootloadTransmitComplete(
                    bootloader::handler::BootloadTransmitComplete::from_le_stream_exact(stream)?,
                ))))
            }
            <bootloader::handler::IncomingBootloadMessage as Identified>::ID => {
                Ok(Self::Handler(Callback::Bootloader(bootloader::handler::Handler::IncomingBootloadMessage(
                    bootloader::handler::IncomingBootloadMessage::from_le_stream_exact(stream)?,
                ))))
            }
            // CBKE responses
            <cbke::dsa_sign::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::DsaSign(
                    cbke::dsa_sign::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::dsa_verify::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::DsaVerify(
                    cbke::dsa_verify::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::dsa_verify283k1::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::DsaVerify283k1(
                    cbke::dsa_verify283k1::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::generate_cbke_keys::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::GenerateCbkeKeys(
                    cbke::generate_cbke_keys::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::generate_cbke_keys283k1::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::GenerateCbkeKeys283k1(
                    cbke::generate_cbke_keys283k1::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::get_certificate::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::GetCertificate(
                    cbke::get_certificate::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::get_certificate283k1::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::GetCertificate283k1(
                    cbke::get_certificate283k1::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::save_preinstalled_cbke_data283k1::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::SavePreinstalledCbkeData283k1(
                    cbke::save_preinstalled_cbke_data283k1::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::set_preinstalled_cbke_data::Response as Identified>::ID => {
                Ok(Self::Response(Response::Cbke(cbke::Response::SetPreinstalledCbkeData(
                    cbke::set_preinstalled_cbke_data::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::CalculateSmacs as Identified>::ID => {
                Ok(Self::Handler(Callback::Cbke(cbke::handler::Handler::CalculateSmacs(
                    cbke::handler::CalculateSmacs::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::CalculateSmacs283k1 as Identified>::ID => {
                Ok(Self::Handler(Callback::Cbke(cbke::handler::Handler::CalculateSmacs283k1(
                    cbke::handler::CalculateSmacs283k1::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::DsaSign as Identified>::ID => {
                Ok(Self::Handler(Callback::Cbke(cbke::handler::Handler::DsaSign(
                    cbke::handler::DsaSign::from_le_stream_exact(stream)?.into(),
                ))))
            }
            <cbke::handler::DsaVerify as Identified>::ID => {
                Ok(Self::Handler(Callback::Cbke(cbke::handler::Handler::DsaVerify(
                    cbke::handler::DsaVerify::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::GenerateCbkeKeys as Identified>::ID => {
                Ok(Self::Handler(Callback::Cbke(cbke::handler::Handler::GenerateCbkeKeys283k1(
                    cbke::handler::GenerateCbkeKeys283k1::from_le_stream_exact(stream)?,
                ))))
            }
            <cbke::handler::GenerateCbkeKeys283k1 as Identified>::ID => {
                Ok(Self::Handler(Callback::Cbke(cbke::handler::Handler::GenerateCbkeKeys283k1(
                    cbke::handler::GenerateCbkeKeys283k1::from_le_stream_exact(stream)?,
                ))))
            }
            // Configuration responses
            <configuration::add_endpoint::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::AddEndpoint(
                    configuration::add_endpoint::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::get_configuration_value::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::GetConfigurationValue(
                    configuration::get_configuration_value::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::get_extended_value::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::GetExtendedValue(
                    configuration::get_extended_value::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::get_policy::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::GetPolicy(
                    configuration::get_policy::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::get_value::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::GetValue(
                    configuration::get_value::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::read_attribute::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::ReadAttribute(
                    configuration::read_attribute::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::send_pan_id_update::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::SendPanIdUpdate(
                    configuration::send_pan_id_update::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::set_configuration_value::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::SetConfigurationValue(
                    configuration::set_configuration_value::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::set_passive_ack_config::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::SetPassiveAckConfig(
                    configuration::set_passive_ack_config::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::set_policy::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::SetPolicy(
                    configuration::set_policy::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::set_value::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::SetValue(
                    configuration::set_value::Response::from_le_stream_exact(stream)?,
                ))))
            }
            VERSION_ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::Version(
                    configuration::version::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <configuration::write_attribute::Response as Identified>::ID => {
                Ok(Self::Response(Response::Configuration(configuration::Response::WriteAttribute(
                    configuration::write_attribute::Response::from_le_stream_exact(stream)?,
                ))))
            }
            // Green Power responses
            <green_power::proxy_table::get_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::ProxyTable(green_power::proxy_table::Response::GetEntry(
                    green_power::proxy_table::get_entry::Response::from_le_stream_exact(stream)?.into(),
                )))))
            }
            <green_power::proxy_table::lookup::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::ProxyTable(green_power::proxy_table::Response::Lookup(
                    green_power::proxy_table::lookup::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::proxy_table::process_gp_pairing::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::ProxyTable(green_power::proxy_table::Response::ProcessGpPairing(
                    green_power::proxy_table::process_gp_pairing::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::send::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::Send(
                    green_power::send::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_commission::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkCommission(
                    green_power::sink_commission::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <green_power::sink_table::clear_all::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::ClearAll(
                    green_power::sink_table::clear_all::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::sink_table::find_or_allocate_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::FindOrAllocateEntry(
                    green_power::sink_table::find_or_allocate_entry::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::sink_table::get_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::GetEntry(
                    green_power::sink_table::get_entry::Response::from_le_stream_exact(stream)?.into(),
                )))))
            }
            <green_power::sink_table::init::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::Init(
                    green_power::sink_table::init::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::sink_table::lookup::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::Lookup(
                    green_power::sink_table::lookup::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::sink_table::number_of_active_entries::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::NumberOfActiveEntries(
                    green_power::sink_table::number_of_active_entries::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::sink_table::remove_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::RemoveEntry(
                    green_power::sink_table::remove_entry::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::sink_table::set_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::GreenPower(green_power::Response::SinkTable(green_power::sink_table::Response::SetEntry(
                    green_power::sink_table::set_entry::Response::from_le_stream_exact(stream)?,
                )))))
            }
            <green_power::handler::IncomingMessage as Identified>::ID => {
                Ok(Self::Handler(Callback::GreenPower(green_power::handler::Handler::IncomingMessage(
                    green_power::handler::IncomingMessage::from_le_stream_exact(stream)?,
                ).into())))
            }
            <green_power::handler::Sent as Identified>::ID => {
                Ok(Self::Handler(Callback::GreenPower(green_power::handler::Handler::Sent(
                    green_power::handler::Sent::from_le_stream_exact(stream)?,
                ).into())))
            }
            // Messaging responses
            <messaging::address_table_entry_is_active::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::AddressTableEntryIsActive(
                    messaging::address_table_entry_is_active::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::get_address_table_remote_eui64::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::GetAddressTableRemoteEui64(
                    messaging::get_address_table_remote_eui64::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::get_address_table_remote_node_id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::GetAddressTableRemoteNodeId(
                    messaging::get_address_table_remote_node_id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::get_beacon_classification_params::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::GetBeaconClassificationParams(
                    messaging::get_beacon_classification_params::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::get_extended_timeout::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::GetExtendedTimeout(
                    messaging::get_extended_timeout::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::get_multicast_table_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::GetMulticastTableEntry(
                    messaging::get_multicast_table_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::lookup_eui64_by_node_id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::LookupEui64ByNodeId(
                    messaging::lookup_eui64_by_node_id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::lookup_node_id_by_eui64::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::LookupNodeIdByEui64(
                    messaging::lookup_node_id_by_eui64::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::maximum_payload_length::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::MaximumPayloadLength(
                    messaging::maximum_payload_length::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::poll_for_data::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::PollForData(
                    messaging::poll_for_data::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::proxy_broadcast::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::ProxyBroadcast(
                    messaging::proxy_broadcast::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::replace_address_table_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::ReplaceAddressTableEntry(
                    messaging::replace_address_table_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::send_broadcast::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SendBroadcast(
                    messaging::send_broadcast::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::send_many_to_one_route_request::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SendManyToOneRouteRequest(
                    messaging::send_many_to_one_route_request::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::send_multicast::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SendMulticast(
                    messaging::send_multicast::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::send_multicast_with_alias::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SendMulticastWithAlias(
                    messaging::send_multicast_with_alias::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::send_raw_message::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SendRawMessage(
                    messaging::send_raw_message::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::send_raw_message_extended::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SendRawMessageExtended(
                    messaging::send_raw_message_extended::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::send_reply::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SendReply(
                    messaging::send_reply::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::send_unicast::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SendUnicast(
                    messaging::send_unicast::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::set_address_table_remote_eui64::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SetAddressTableRemoteEui64(
                    messaging::set_address_table_remote_eui64::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::set_address_table_remote_node_id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SetAddressTableRemoteNodeId(
                    messaging::set_address_table_remote_node_id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::set_beacon_classification_params::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SetBeaconClassificationParams(
                    messaging::set_beacon_classification_params::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::set_extended_timeout::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SetExtendedTimeout(
                    messaging::set_extended_timeout::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::set_mac_poll_failure_wait_time::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SetMacPollFailureWaitTime(
                    messaging::set_mac_poll_failure_wait_time::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::set_multicast_table_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SetMulticastTableEntry(
                    messaging::set_multicast_table_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::set_source_route_discovery_mode::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::SetSourceRouteDiscoveryMode(
                    messaging::set_source_route_discovery_mode::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::unicast_current_network_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::UnicastCurrentNetworkKey(
                    messaging::unicast_current_network_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::write_node_data::Response as Identified>::ID => {
                Ok(Self::Response(Response::Messaging(messaging::Response::WriteNodeData(
                    messaging::write_node_data::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <messaging::handler::IdConflict as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::IdConflict(
                    messaging::handler::IdConflict::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::IncomingManyToOneRouteRequest as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::IncomingManyToOneRouteRequest(
                    messaging::handler::IncomingManyToOneRouteRequest::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::IncomingMessage as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::IncomingMessage(
                    messaging::handler::IncomingMessage::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::IncomingNetworkStatus as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::IncomingNetworkStatus(
                    messaging::handler::IncomingNetworkStatus::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::IncomingRouteError as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::IncomingRouteError(
                    messaging::handler::IncomingRouteError::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::IncomingRouteRecord as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::IncomingRouteRecord(
                    messaging::handler::IncomingRouteRecord::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::IncomingSenderEui64 as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::IncomingSenderEui64(
                    messaging::handler::IncomingSenderEui64::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::MacFilterMatchMessage as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::MacFilterMatchMessage(
                    messaging::handler::MacFilterMatchMessage::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::MacPassthroughMessage as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::MacPassthroughMessage(
                    messaging::handler::MacPassthroughMessage::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::MessageSent as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::MessageSent(
                    messaging::handler::MessageSent::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::Poll as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::Poll(
                    messaging::handler::Poll::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::PollComplete as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::PollComplete(
                    messaging::handler::PollComplete::from_le_stream_exact(stream)?,
                ).into())))
            }
            <messaging::handler::RawTransmitComplete as Identified>::ID => {
                Ok(Self::Handler(Callback::Messaging(messaging::handler::Handler::RawTransmitComplete(
                    messaging::handler::RawTransmitComplete::from_le_stream_exact(stream)?,
                ).into())))
            }
            // MfgLib responses
            <mfglib::end::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::End(
                    mfglib::end::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::get_channel::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::GetChannel(
                    mfglib::get_channel::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::get_power::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::GetPower(
                    mfglib::get_power::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::send_packet::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::SendPacket(
                    mfglib::send_packet::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::set_channel::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::SetChannel(
                    mfglib::set_channel::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::set_power::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::SetPower(
                    mfglib::set_power::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::start::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::Start(
                    mfglib::start::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::start_stream::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::StartStream(
                    mfglib::start_stream::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::start_tone::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::StartTone(
                    mfglib::start_tone::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::stop_stream::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::StopStream(
                    mfglib::stop_stream::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::stop_tone::Response as Identified>::ID => {
                Ok(Self::Response(Response::MfgLib(mfglib::Response::StopTone(
                    mfglib::stop_tone::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <mfglib::handler::Rx as Identified>::ID => {
                Ok(Self::Handler(Callback::MfgLib(mfglib::handler::Handler::Rx(
                    mfglib::handler::Rx::from_le_stream_exact(stream)?,
                ).into())))
            }
            // Networking responses
            <networking::child_id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::ChildId(
                    networking::child_id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::clear_stored_beacons::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::ClearStoredBeacons(
                    networking::clear_stored_beacons::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::energy_scan_request::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::EnergyScanRequest(
                    networking::energy_scan_request::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::find_and_rejoin_network::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::FindAndRejoinNetwork(
                    networking::find_and_rejoin_network::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::find_unused_pan_id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::FindUnusedPanId(
                    networking::find_unused_pan_id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::form_network::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::FormNetwork(
                    networking::form_network::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_child_data::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetChildData(
                    networking::get_child_data::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_current_duty_cycle::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetCurrentDutyCycle(
                    networking::get_current_duty_cycle::Response::from_le_stream_exact(stream)?.into(),
                ))))
            }
            <networking::get_duty_cycle_limits::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetDutyCycleLimits(
                    networking::get_duty_cycle_limits::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_duty_cycle_state::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetDutyCycleState(
                    networking::get_duty_cycle_state::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_first_beacon::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetFirstBeacon(
                    networking::get_first_beacon::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_logical_channel::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetLogicalChannel(
                    networking::get_logical_channel::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_neighbor::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetNeighbor(
                    networking::get_neighbor::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_neighbor_frame_counter::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetNeighborFrameCounter(
                    networking::get_neighbor_frame_counter::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_network_parameters::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetNetworkParameters(
                    networking::get_network_parameters::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_next_beacon::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetNextBeacon(
                    networking::get_next_beacon::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_num_stored_beacons::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetNumStoredBeacons(
                    networking::get_num_stored_beacons::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_parent_child_parameters::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetParentChildParameters(
                    networking::get_parent_child_parameters::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_radio_channel::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetRadioChannel(
                    networking::get_radio_channel::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_radio_parameters::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetRadioParameters(
                    networking::get_radio_parameters::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_route_table_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetRouteTableEntry(
                    networking::get_route_table_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_routing_shortcut_threshold::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetRoutingShortcutThreshold(
                    networking::get_routing_shortcut_threshold::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_source_route_table_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetSourceRouteTableEntry(
                    networking::get_source_route_table_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_source_route_table_filled_size::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetSourceRouteTableFilledSize(
                    networking::get_source_route_table_filled_size::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::get_source_route_table_total_size::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::GetSourceRouteTableTotalSize(
                    networking::get_source_route_table_total_size::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::Id(
                    networking::id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::join_network::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::JoinNetwork(
                    networking::join_network::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::join_network_directly::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::JoinNetworkDirectly(
                    networking::join_network_directly::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::leave_network::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::LeaveNetwork(
                    networking::leave_network::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::multi_phy_set_radio_channel::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::MultiPhySetRadioChannel(
                    networking::multi_phy_set_radio_channel::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::multi_phy_set_radio_power::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::MultiPhySetRadioPower(
                    networking::multi_phy_set_radio_power::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::multi_phy_start::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::MultiPhyStart(
                    networking::multi_phy_start::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::multi_phy_stop::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::MultiPhyStop(
                    networking::multi_phy_stop::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::neighbor_count::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::NeighborCount(
                    networking::neighbor_count::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::network_init::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::NetworkInit(
                    networking::network_init::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::network_state::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::NetworkState(
                    networking::network_state::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::permit_joining::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::PermitJoining(
                    networking::permit_joining::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::send_link_power_delta_request::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SendLinkPowerDeltaRequest(
                    networking::send_link_power_delta_request::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_broken_route_error_code::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetBrokenRouteErrorCode(
                    networking::set_broken_route_error_code::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_child_data::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetChildData(
                    networking::set_child_data::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_concentrator::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetConcentrator(
                    networking::set_concentrator::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_duty_cycle_limits_in_stack::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetDutyCycleLimitsInStack(
                    networking::set_duty_cycle_limits_in_stack::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_logical_and_radio_channel::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetLogicalAndRadioChannel(
                    networking::set_logical_and_radio_channel::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_manufacturer_code::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetManufacturerCode(networking::set_manufacturer_code::Response::from_le_stream_exact(stream)?))))
            }
            <networking::set_neighbor_frame_counter::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetNeighborFrameCounter(
                    networking::set_neighbor_frame_counter::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_power_descriptor::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetPowerDescriptor(
                    networking::set_power_descriptor::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_radio_channel::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetRadioChannel(
                    networking::set_radio_channel::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_radio_ieee802154_cca_mode::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetRadioIeee802154CcaMode(
                    networking::set_radio_ieee802154_cca_mode::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_radio_power::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetRadioPower(
                    networking::set_radio_power::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::set_routing_shortcut_threshold::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::SetRoutingShortcutThreshold(
                    networking::set_routing_shortcut_threshold::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::start_scan::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::StartScan(
                    networking::start_scan::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::stop_scan::Response as Identified>::ID => {
                Ok(Self::Response(Response::Networking(networking::Response::StopScan(
                    networking::stop_scan::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::ChildJoin as Identified>::ID => {
                Ok(Self::Handler(Callback::Networking(networking::handler::Handler::ChildJoin(
                    networking::handler::ChildJoin::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::DutyCycle as Identified>::ID => {
                Ok(Self::Handler(Callback::Networking(networking::handler::Handler::DutyCycle(
                    networking::handler::DutyCycle::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::EnergyScanResult as Identified>::ID => {
                Ok(Self::Handler(Callback::Networking(networking::handler::Handler::EnergyScanResult(
                    networking::handler::EnergyScanResult::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::NetworkFound as Identified>::ID => {
                Ok(Self::Handler(Callback::Networking(networking::handler::Handler::NetworkFound(
                    networking::handler::NetworkFound::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::ScanComplete as Identified>::ID => {
                Ok(Self::Handler(Callback::Networking(networking::handler::Handler::ScanComplete(
                    networking::handler::ScanComplete::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::StackStatus as Identified>::ID => {
                Ok(Self::Handler(Callback::Networking(networking::handler::Handler::StackStatus(
                    networking::handler::StackStatus::from_le_stream_exact(stream)?,
                ))))
            }
            <networking::handler::UnusedPanIdFound as Identified>::ID => {
                Ok(Self::Handler(Callback::Networking(networking::handler::Handler::UnusedPanIdFound(
                    networking::handler::UnusedPanIdFound::from_le_stream_exact(stream)?,
                ))))
            }
            // Security responses
            <security::check_key_context::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::CheckKeyContext(
                    security::check_key_context::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::clear_key_table::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ClearKeyTable(
                    security::clear_key_table::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::clear_transient_link_keys::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ClearTransientLinkKeys(
                    security::clear_transient_link_keys::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::erase_key_table_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::EraseKeyTableEntry(
                    security::erase_key_table_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::export_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ExportKey(
                    security::export_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::export_link_key_by_eui::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ExportLinkKeyByEui(
                    security::export_link_key_by_eui::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::export_link_key_by_index::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ExportLinkKeyByIndex(
                    security::export_link_key_by_index::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::export_transient_key::by_eui::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ExportTransientKeyByEui(
                    security::export_transient_key::by_eui::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::export_transient_key::by_index::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ExportTransientKeyByIndex(
                    security::export_transient_key::by_index::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::find_key_table_entry::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::FindKeyTableEntry(
                    security::find_key_table_entry::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::get_aps_key_info::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::GetApsKeyInfo(
                    security::get_aps_key_info::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::get_current_security_state::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::GetCurrentSecurityState(
                    security::get_current_security_state::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::get_network_key_info::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::GetNetworkKeyInfo(
                    security::get_network_key_info::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::import_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ImportKey(
                    security::import_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::import_link_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ImportLinkKey(
                    security::import_link_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::import_transient_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::ImportTransientKey(
                    security::import_transient_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::request_link_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::RequestLinkKey(
                    security::request_link_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::send_trust_center_link_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::SendTrustCenterLinkKey(
                    security::send_trust_center_link_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::set_initial_security_state::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::SetInitialSecurityState(
                    security::set_initial_security_state::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::update_tc_link_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Security(security::Response::UpdateTcLinkKey(
                    security::update_tc_link_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <security::handler::SwitchNetworkKey as Identified>::ID => {
                Ok(Self::Handler(Callback::Security(security::handler::Handler::SwitchNetworkKey(
                    security::handler::SwitchNetworkKey::from_le_stream_exact(stream)?,
                ))))
            }
            <security::handler::ZigbeeKeyEstablishment as Identified>::ID => {
                Ok(Self::Handler(Callback::Security(security::handler::Handler::ZigbeeKeyEstablishment(
                    security::handler::ZigbeeKeyEstablishment::from_le_stream_exact(stream)?.into(),
                ))))
            }
            // Token interface responses
            <token_interface::get_token_count::Response as Identified>::ID => {
                Ok(Self::Response(Response::TokenInterface(token_interface::Response::GetTokenCount(
                    token_interface::get_token_count::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <token_interface::get_token_data::Response as Identified>::ID => {
                Ok(Self::Response(Response::TokenInterface(token_interface::Response::GetTokenData(
                    token_interface::get_token_data::Response::from_le_stream_exact(stream)?.into(),
                ))))
            }
            <token_interface::get_token_info::Response as Identified>::ID => {
                Ok(Self::Response(Response::TokenInterface(token_interface::Response::GetTokenInfo(
                    token_interface::get_token_info::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <token_interface::gp_security_test_vectors::Response as Identified>::ID => {
                Ok(Self::Response(Response::TokenInterface(token_interface::Response::GpSecurityTestVectors(
                    token_interface::gp_security_test_vectors::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <token_interface::reset_node::Response as Identified>::ID => {
                Ok(Self::Response(Response::TokenInterface(token_interface::Response::ResetNode(
                    token_interface::reset_node::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <token_interface::set_token_data::Response as Identified>::ID => {
                Ok(Self::Response(Response::TokenInterface(token_interface::Response::SetTokenData(
                    token_interface::set_token_data::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <token_interface::token_factory_reset::Response as Identified>::ID => {
                Ok(Self::Response(Response::TokenInterface(token_interface::Response::TokenFactoryReset(
                    token_interface::token_factory_reset::Response::from_le_stream_exact(stream)?,
                ))))
            }
            // Trust Center responses
            <trust_center::aes_mmo_hash::Response as Identified>::ID => {
                Ok(Self::Response(Response::TrustCenter(trust_center::Response::AesMmoHash(
                    trust_center::aes_mmo_hash::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <trust_center::broadcast_network_key_switch::Response as Identified>::ID => {
                Ok(Self::Response(Response::TrustCenter(trust_center::Response::BroadcastNetworkKeySwitch(
                    trust_center::broadcast_network_key_switch::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <trust_center::broadcast_next_network_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::TrustCenter(trust_center::Response::BroadcastNextNetworkKey(
                    trust_center::broadcast_next_network_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <trust_center::remove_device::Response as Identified>::ID => {
                Ok(Self::Response(Response::TrustCenter(trust_center::Response::RemoveDevice(
                    trust_center::remove_device::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <trust_center::unicast_nwk_key_update::Response as Identified>::ID => {
                Ok(Self::Response(Response::TrustCenter(trust_center::Response::UnicastNwkKeyUpdate(
                    trust_center::unicast_nwk_key_update::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <trust_center::handler::TrustCenterJoin as Identified>::ID => {
                Ok(Self::Handler(Callback::TrustCenter(trust_center::handler::Handler::TrustCenterJoin(
                    trust_center::handler::TrustCenterJoin::from_le_stream_exact(stream)?,
                ))))
            }
            // Utility responses
            <utilities::custom_frame::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::CustomFrame(
                    utilities::custom_frame::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::debug_write::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::DebugWrite(
                    utilities::debug_write::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::delay_test::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::DelayTest(
                    utilities::delay_test::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::echo::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::Echo(
                    utilities::echo::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_eui64::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetEui64(
                    utilities::get_eui64::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_library_status::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetLibraryStatus(
                    utilities::get_library_status::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_mfg_token::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetMfgToken(
                    utilities::get_mfg_token::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_node_id::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetNodeId(
                    utilities::get_node_id::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_phy_interface_count::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetPhyInterfaceCount(
                    utilities::get_phy_interface_count::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_random_number::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetRandomNumber(
                    utilities::get_random_number::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_timer::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetTimer(
                    utilities::get_timer::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_token::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetToken(
                    utilities::get_token::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_true_random_entropy_source::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetTrueRandomEntropySource(
                    utilities::get_true_random_entropy_source::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::get_xncp_info::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::GetXncpInfo(
                    utilities::get_xncp_info::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::invalid_command::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::InvalidCommand(
                    utilities::invalid_command::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::no_callbacks::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::NoCallbacks(
                    utilities::no_callbacks::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::nop::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::Nop(
                    utilities::nop::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::read_and_clear_counters::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::ReadAndClearCounters(
                    utilities::read_and_clear_counters::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::read_counters::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::ReadCounters(
                    utilities::read_counters::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::set_mfg_token::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::SetMfgToken(
                    utilities::set_mfg_token::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::set_timer::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::SetTimer(
                    utilities::set_timer::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::set_token::Response as Identified>::ID => {
                Ok(Self::Response(Response::Utilities(utilities::Response::SetToken(
                    utilities::set_token::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::handler::CounterRollover as Identified>::ID => {
                Ok(Self::Handler(Callback::Utilities(utilities::handler::Handler::CounterRollover(
                    utilities::handler::CounterRollover::from_le_stream_exact(stream)?,
                ))))
            }
            <utilities::handler::CustomFrame as Identified>::ID => {
                Ok(Self::Handler(Callback::Utilities(utilities::handler::Handler::CustomFrame(
                    utilities::handler::CustomFrame::from_le_stream_exact(stream)?.into(),
                ))))
            }
            <utilities::handler::StackTokenChanged as Identified>::ID => {
                Ok(Self::Handler(Callback::Utilities(
                    utilities::handler::Handler::StackTokenChanged(
                        utilities::handler::StackTokenChanged::from_le_stream_exact(stream)?,
                    ))))
            }
            <utilities::handler::Timer as Identified>::ID => {
                Ok(Self::Handler(Callback::Utilities(utilities::handler::Handler::Timer(
                    utilities::handler::Timer::from_le_stream_exact(stream)?,
                ))))
            }
            // WWAH responses
            <wwah::get_parent_classification_enabled::Response as Identified>::ID => {
                Ok(Self::Response(Response::Wwah(wwah::Response::GetParentClassificationEnabled(
                    wwah::get_parent_classification_enabled::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <wwah::is_hub_connected::Response as Identified>::ID => {
                Ok(Self::Response(Response::Wwah(wwah::Response::IsHubConnected(
                    wwah::is_hub_connected::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <wwah::is_uptime_long::Response as Identified>::ID => {
                Ok(Self::Response(Response::Wwah(wwah::Response::IsUptimeLong(
                    wwah::is_uptime_long::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <wwah::set_hub_connectivity::Response as Identified>::ID => {
                Ok(Self::Response(Response::Wwah(wwah::Response::SetHubConnectivity(
                    wwah::set_hub_connectivity::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <wwah::set_long_uptime::Response as Identified>::ID => {
                Ok(Self::Response(Response::Wwah(wwah::Response::SetLongUptime(
                    wwah::set_long_uptime::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <wwah::set_parent_classification_enabled::Response as Identified>::ID => {
                Ok(Self::Response(Response::Wwah(wwah::Response::SetParentClassificationEnabled(
                    wwah::set_parent_classification_enabled::Response::from_le_stream_exact(stream)?,
                ))))
            }
            // ZLL responses
            <zll::clear_tokens::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::ClearTokens(
                    zll::clear_tokens::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::get_primary_channel_mask::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::GetPrimaryChannelMask(
                    zll::get_primary_channel_mask::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::get_secondary_channel_mask::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::GetSecondaryChannelMask(
                    zll::get_secondary_channel_mask::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::get_tokens::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::GetTokens(
                    zll::get_tokens::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::is_zll_network::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::IsZllNetwork(
                    zll::is_zll_network::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::network_ops::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::NetworkOps(
                    zll::network_ops::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::operation_in_progress::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::OperationInProgress(
                    zll::operation_in_progress::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::rx_on_when_idle_get_active::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::RxOnWhenIdleGetActive(
                    zll::rx_on_when_idle_get_active::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_additional_state::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetAdditionalState(
                    zll::set_additional_state::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_data_token::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetDataToken(
                    zll::set_data_token::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_initial_security_state::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetInitialSecurityState(
                    zll::set_initial_security_state::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_node_type::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetNodeType(
                    zll::set_node_type::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_non_zll_network::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetNonZllNetwork(
                    zll::set_non_zll_network::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_primary_channel_mask::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetPrimaryChannelMask(
                    zll::set_primary_channel_mask::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_radio_idle_mode::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetRadioIdleMode(
                    zll::set_radio_idle_mode::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_rx_on_when_idle::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetRxOnWhenIdle(
                    zll::set_rx_on_when_idle::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_secondary_channel_mask::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetSecondaryChannelMask(
                    zll::set_secondary_channel_mask::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::set_security_state_without_key::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::SetSecurityStateWithoutKey(
                    zll::set_security_state_without_key::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::start_scan::Response as Identified>::ID => {
                Ok(Self::Response(Response::Zll(zll::Response::StartScan(
                    zll::start_scan::Response::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::handler::AddressAssignment as Identified>::ID => {
                Ok(Self::Handler(Callback::Zll(zll::handler::Handler::AddressAssignment(
                    zll::handler::AddressAssignment::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::handler::NetworkFound as Identified>::ID => {
                Ok(Self::Handler(Callback::Zll(zll::handler::Handler::NetworkFound(
                    zll::handler::NetworkFound::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::handler::ScanComplete as Identified>::ID => {
                Ok(Self::Handler(Callback::Zll(zll::handler::Handler::ScanComplete(
                    zll::handler::ScanComplete::from_le_stream_exact(stream)?,
                ))))
            }
            <zll::handler::TouchLinkTarget as Identified>::ID => {
                Ok(Self::Handler(Callback::Zll(zll::handler::Handler::TouchLinkTarget(
                    zll::handler::TouchLinkTarget::from_le_stream_exact(stream)?,
                ))))
            }
            _ => Err(Decode::InvalidFrameId(id))
        }
    }
}
