//! Messaging Frames

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
