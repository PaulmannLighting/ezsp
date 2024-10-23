//! Messaging Frames

pub(crate) mod address_table_entry_is_active;
pub(crate) mod get_address_table_remote_eui64;
pub(crate) mod get_address_table_remote_node_id;
pub(crate) mod get_beacon_classification_params;
pub(crate) mod get_extended_timeout;
pub(crate) mod get_multicast_table_entry;
pub mod handler;
pub(crate) mod lookup_eui64_by_node_id;
pub(crate) mod lookup_node_id_by_eui64;
pub(crate) mod maximum_payload_length;
pub(crate) mod poll_for_data;
pub(crate) mod proxy_broadcast;
pub(crate) mod replace_address_table_entry;
pub(crate) mod send_broadcast;
pub(crate) mod send_many_to_one_route_request;
pub(crate) mod send_multicast;
pub(crate) mod send_multicast_with_alias;
pub(crate) mod send_raw_message;
pub(crate) mod send_raw_message_extended;
pub(crate) mod send_reply;
pub(crate) mod send_unicast;
pub(crate) mod set_address_table_remote_eui64;
pub(crate) mod set_address_table_remote_node_id;
pub(crate) mod set_beacon_classification_params;
pub(crate) mod set_extended_timeout;
pub(crate) mod set_mac_poll_failure_wait_time;
pub(crate) mod set_multicast_table_entry;
pub(crate) mod set_source_route_discovery_mode;
pub(crate) mod unicast_current_network_key;
pub(crate) mod write_node_data;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    AddressTableEntryIsActive(address_table_entry_is_active::Command),
    GetAddressTableRemoteEui64(get_address_table_remote_eui64::Command),
    GetAddressTableRemoteNodeId(get_address_table_remote_node_id::Command),
    GetBeaconClassificationParams(get_beacon_classification_params::Command),
    GetExtendedTimeout(get_extended_timeout::Command),
    GetMulticastTableEntry(get_multicast_table_entry::Command),
    LookupEui64ByNodeId(lookup_eui64_by_node_id::Command),
    LookupNodeIdByEui64(lookup_node_id_by_eui64::Command),
    MaximumPayloadLength(maximum_payload_length::Command),
    PollForData(poll_for_data::Command),
    ProxyBroadcast(proxy_broadcast::Command),
    ReplaceAddressTableEntry(replace_address_table_entry::Command),
    SendBroadcast(send_broadcast::Command),
    SendManyToOneRouteRequest(send_many_to_one_route_request::Command),
    SendMulticast(send_multicast::Command),
    SendMulticastWithAlias(send_multicast_with_alias::Command),
    SendRawMessage(send_raw_message::Command),
    SendRawMessageExtended(send_raw_message_extended::Command),
    SendReply(send_reply::Command),
    SendUnicast(send_unicast::Command),
    SetAddressTableRemoteEui64(set_address_table_remote_eui64::Command),
    SetAddressTableRemoteNodeId(set_address_table_remote_node_id::Command),
    SetBeaconClassificationParams(set_beacon_classification_params::Command),
    SetExtendedTimeout(set_extended_timeout::Command),
    SetMacPollFailureWaitTime(set_mac_poll_failure_wait_time::Command),
    SetMulticastTableEntry(set_multicast_table_entry::Command),
    SetSourceRouteDiscoveryMode(set_source_route_discovery_mode::Command),
    UnicastCurrentNetworkKey(unicast_current_network_key::Command),
    WriteNodeData(write_node_data::Command),
}

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
