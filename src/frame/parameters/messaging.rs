//! Messaging Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

pub mod address_table_entry_is_active;
pub mod get_address_table_remote_eui64;
pub mod get_address_table_remote_node_id;
pub mod get_beacon_classification_params;
pub mod get_extended_timeout;
pub mod get_multicast_table_entry;
pub mod handler;
pub mod lookup_eui64_by_node_id;
pub mod lookup_node_id_by_eui64;
pub mod maximum_payload_length;
pub mod poll_for_data;
pub mod proxy_broadcast;
pub mod replace_address_table_entry;
pub mod send_broadcast;
pub mod send_many_to_one_route_request;
pub mod send_multicast;
pub mod send_multicast_with_alias;
pub mod send_raw_message;
pub mod send_raw_message_extended;
pub mod send_reply;
pub mod send_unicast;
pub mod set_address_table_remote_eui64;
pub mod set_address_table_remote_node_id;
pub mod set_beacon_classification_params;
pub mod set_extended_timeout;
pub mod set_mac_poll_failure_wait_time;
pub mod set_multicast_table_entry;
pub mod set_source_route_discovery_mode;
pub mod unicast_current_network_key;
pub mod write_node_data;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    AddressTableEntryIsActive(address_table_entry_is_active::Response),
    GetAddressTableRemoteEui64(get_address_table_remote_eui64::Response),
    GetAddressTableRemoteNodeId(get_address_table_remote_node_id::Response),
    GetBeaconClassificationParams(get_beacon_classification_params::Response),
    GetExtendedTimeout(get_extended_timeout::Response),
    GetMulticastTableEntry(get_multicast_table_entry::Response),
    LookupEui64ByNodeId(lookup_eui64_by_node_id::Response),
    LookupNodeIdByEui64(lookup_node_id_by_eui64::Response),
    MaximumPayloadLength(maximum_payload_length::Response),
    PollForData(poll_for_data::Response),
    ProxyBroadcast(proxy_broadcast::Response),
    ReplaceAddressTableEntry(replace_address_table_entry::Response),
    SendBroadcast(send_broadcast::Response),
    SendManyToOneRouteRequest(send_many_to_one_route_request::Response),
    SendMulticast(send_multicast::Response),
    SendMulticastWithAlias(send_multicast_with_alias::Response),
    SendRawMessage(send_raw_message::Response),
    SendRawMessageExtended(send_raw_message_extended::Response),
    SendReply(send_reply::Response),
    SendUnicast(send_unicast::Response),
    SetAddressTableRemoteEui64(set_address_table_remote_eui64::Response),
    SetAddressTableRemoteNodeId(set_address_table_remote_node_id::Response),
    SetBeaconClassificationParams(set_beacon_classification_params::Response),
    SetExtendedTimeout(set_extended_timeout::Response),
    SetMacPollFailureWaitTime(set_mac_poll_failure_wait_time::Response),
    SetMulticastTableEntry(set_multicast_table_entry::Response),
    SetSourceRouteDiscoveryMode(set_source_route_discovery_mode::Response),
    UnicastCurrentNetworkKey(unicast_current_network_key::Response),
    WriteNodeData(write_node_data::Response),
    Handler(handler::Handler),
}

impl Parsable for Response {
    #[allow(clippy::too_many_lines)]
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <address_table_entry_is_active::Response as Parameter>::ID => {
                Ok(Self::AddressTableEntryIsActive(
                    address_table_entry_is_active::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_address_table_remote_eui64::Response as Parameter>::ID => {
                Ok(Self::GetAddressTableRemoteEui64(
                    get_address_table_remote_eui64::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_address_table_remote_node_id::Response as Parameter>::ID => {
                Ok(Self::GetAddressTableRemoteNodeId(
                    get_address_table_remote_node_id::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_beacon_classification_params::Response as Parameter>::ID => {
                Ok(Self::GetBeaconClassificationParams(
                    get_beacon_classification_params::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_extended_timeout::Response as Parameter>::ID => Ok(Self::GetExtendedTimeout(
                get_extended_timeout::Response::from_le_stream_exact(stream)?,
            )),
            <get_multicast_table_entry::Response as Parameter>::ID => {
                Ok(Self::GetMulticastTableEntry(
                    get_multicast_table_entry::Response::from_le_stream_exact(stream)?,
                ))
            }
            <lookup_eui64_by_node_id::Response as Parameter>::ID => Ok(Self::LookupEui64ByNodeId(
                lookup_eui64_by_node_id::Response::from_le_stream_exact(stream)?,
            )),
            <lookup_node_id_by_eui64::Response as Parameter>::ID => Ok(Self::LookupNodeIdByEui64(
                lookup_node_id_by_eui64::Response::from_le_stream_exact(stream)?,
            )),
            <maximum_payload_length::Response as Parameter>::ID => Ok(Self::MaximumPayloadLength(
                maximum_payload_length::Response::from_le_stream_exact(stream)?,
            )),
            <poll_for_data::Response as Parameter>::ID => Ok(Self::PollForData(
                poll_for_data::Response::from_le_stream_exact(stream)?,
            )),
            <proxy_broadcast::Response as Parameter>::ID => Ok(Self::ProxyBroadcast(
                proxy_broadcast::Response::from_le_stream_exact(stream)?,
            )),
            <replace_address_table_entry::Response as Parameter>::ID => {
                Ok(Self::ReplaceAddressTableEntry(
                    replace_address_table_entry::Response::from_le_stream_exact(stream)?,
                ))
            }
            <send_broadcast::Response as Parameter>::ID => Ok(Self::SendBroadcast(
                send_broadcast::Response::from_le_stream_exact(stream)?,
            )),
            <send_many_to_one_route_request::Response as Parameter>::ID => {
                Ok(Self::SendManyToOneRouteRequest(
                    send_many_to_one_route_request::Response::from_le_stream_exact(stream)?,
                ))
            }
            <send_multicast::Response as Parameter>::ID => Ok(Self::SendMulticast(
                send_multicast::Response::from_le_stream_exact(stream)?,
            )),
            <send_multicast_with_alias::Response as Parameter>::ID => {
                Ok(Self::SendMulticastWithAlias(
                    send_multicast_with_alias::Response::from_le_stream_exact(stream)?,
                ))
            }
            <send_raw_message::Response as Parameter>::ID => Ok(Self::SendRawMessage(
                send_raw_message::Response::from_le_stream_exact(stream)?,
            )),
            <send_raw_message_extended::Response as Parameter>::ID => {
                Ok(Self::SendRawMessageExtended(
                    send_raw_message_extended::Response::from_le_stream_exact(stream)?,
                ))
            }
            <send_reply::Response as Parameter>::ID => Ok(Self::SendReply(
                send_reply::Response::from_le_stream_exact(stream)?,
            )),
            <send_unicast::Response as Parameter>::ID => Ok(Self::SendUnicast(
                send_unicast::Response::from_le_stream_exact(stream)?,
            )),
            <set_address_table_remote_eui64::Response as Parameter>::ID => {
                Ok(Self::SetAddressTableRemoteEui64(
                    set_address_table_remote_eui64::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_address_table_remote_node_id::Response as Parameter>::ID => {
                Ok(Self::SetAddressTableRemoteNodeId(
                    set_address_table_remote_node_id::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_beacon_classification_params::Response as Parameter>::ID => {
                Ok(Self::SetBeaconClassificationParams(
                    set_beacon_classification_params::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_extended_timeout::Response as Parameter>::ID => Ok(Self::SetExtendedTimeout(
                set_extended_timeout::Response::from_le_stream_exact(stream)?,
            )),
            <set_mac_poll_failure_wait_time::Response as Parameter>::ID => {
                Ok(Self::SetMacPollFailureWaitTime(
                    set_mac_poll_failure_wait_time::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_multicast_table_entry::Response as Parameter>::ID => {
                Ok(Self::SetMulticastTableEntry(
                    set_multicast_table_entry::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_source_route_discovery_mode::Response as Parameter>::ID => {
                Ok(Self::SetSourceRouteDiscoveryMode(
                    set_source_route_discovery_mode::Response::from_le_stream_exact(stream)?,
                ))
            }
            <unicast_current_network_key::Response as Parameter>::ID => {
                Ok(Self::UnicastCurrentNetworkKey(
                    unicast_current_network_key::Response::from_le_stream_exact(stream)?,
                ))
            }
            <write_node_data::Response as Parameter>::ID => Ok(Self::WriteNodeData(
                write_node_data::Response::from_le_stream_exact(stream)?,
            )),
            <handler::IdConflict as Parameter>::ID => Ok(Self::Handler(
                handler::IdConflict::from_le_stream_exact(stream)?.into(),
            )),
            <handler::IncomingManyToOneRouteRequest as Parameter>::ID => Ok(Self::Handler(
                handler::IncomingManyToOneRouteRequest::from_le_stream_exact(stream)?.into(),
            )),
            <handler::IncomingMessage as Parameter>::ID => Ok(Self::Handler(
                handler::IncomingMessage::from_le_stream_exact(stream)?.into(),
            )),
            <handler::IncomingNetworkStatus as Parameter>::ID => Ok(Self::Handler(
                handler::IncomingNetworkStatus::from_le_stream_exact(stream)?.into(),
            )),
            <handler::IncomingRouteError as Parameter>::ID => Ok(Self::Handler(
                handler::IncomingRouteError::from_le_stream_exact(stream)?.into(),
            )),
            <handler::IncomingSenderEui64 as Parameter>::ID => Ok(Self::Handler(
                handler::IncomingSenderEui64::from_le_stream_exact(stream)?.into(),
            )),
            <handler::MacFilterMatchMessage as Parameter>::ID => Ok(Self::Handler(
                handler::MacFilterMatchMessage::from_le_stream_exact(stream)?.into(),
            )),
            <handler::MacPassthroughMessage as Parameter>::ID => Ok(Self::Handler(
                handler::MacPassthroughMessage::from_le_stream_exact(stream)?.into(),
            )),
            <handler::MessageSent as Parameter>::ID => Ok(Self::Handler(
                handler::MessageSent::from_le_stream_exact(stream)?.into(),
            )),
            <handler::Poll as Parameter>::ID => Ok(Self::Handler(
                handler::Poll::from_le_stream_exact(stream)?.into(),
            )),
            <handler::PollComplete as Parameter>::ID => Ok(Self::Handler(
                handler::PollComplete::from_le_stream_exact(stream)?.into(),
            )),
            <handler::RawTransmitComplete as Parameter>::ID => Ok(Self::Handler(
                handler::RawTransmitComplete::from_le_stream_exact(stream)?.into(),
            )),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
