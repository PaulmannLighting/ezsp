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

/// Messaging response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response to the `address_table_entry_is_active` command.
    AddressTableEntryIsActive(address_table_entry_is_active::Response),
    /// Response to the `get_address_table_remote_eui64` command.
    GetAddressTableRemoteEui64(get_address_table_remote_eui64::Response),
    /// Response to the `get_address_table_remote_node_id` command.
    GetAddressTableRemoteNodeId(get_address_table_remote_node_id::Response),
    /// Response to the `get_beacon_classification_params` command.
    GetBeaconClassificationParams(get_beacon_classification_params::Response),
    /// Response to the `get_extended_timeout` command.
    GetExtendedTimeout(get_extended_timeout::Response),
    /// Response to the `get_multicast_table_entry` command.
    GetMulticastTableEntry(get_multicast_table_entry::Response),
    /// Response to the `lookup_eui64_by_node_id` command.
    LookupEui64ByNodeId(lookup_eui64_by_node_id::Response),
    /// Response to the `lookup_node_id_by_eui64` command.
    LookupNodeIdByEui64(lookup_node_id_by_eui64::Response),
    /// Response to the `maximum_payload_length` command.
    MaximumPayloadLength(maximum_payload_length::Response),
    /// Response to the `poll_for_data` command.
    PollForData(poll_for_data::Response),
    /// Response to the `proxy_broadcast` command.
    ProxyBroadcast(proxy_broadcast::Response),
    /// Response to the `replace_address_table_entry` command.
    ReplaceAddressTableEntry(replace_address_table_entry::Response),
    /// Response to the `send_broadcast` command.
    SendBroadcast(send_broadcast::Response),
    /// Response to the `send_many_to_one_route_request` command.
    SendManyToOneRouteRequest(send_many_to_one_route_request::Response),
    /// Response to the `send_multicast` command.
    SendMulticast(send_multicast::Response),
    /// Response to the `send_multicast_with_alias` command.
    SendMulticastWithAlias(send_multicast_with_alias::Response),
    /// Response to the `send_raw_message` command.
    SendRawMessage(send_raw_message::Response),
    /// Response to the `send_raw_message_extended` command.
    SendRawMessageExtended(send_raw_message_extended::Response),
    /// Response to the `send_reply` command.
    SendReply(send_reply::Response),
    /// Response to the `send_unicast` command.
    SendUnicast(send_unicast::Response),
    /// Response to the `set_address_table_remote_eui64` command.
    SetAddressTableRemoteEui64(set_address_table_remote_eui64::Response),
    /// Response to the `set_address_table_remote_node_id` command.
    SetAddressTableRemoteNodeId(set_address_table_remote_node_id::Response),
    /// Response to the `set_beacon_classification_params` command.
    SetBeaconClassificationParams(set_beacon_classification_params::Response),
    /// Response to the `set_extended_timeout` command.
    SetExtendedTimeout(set_extended_timeout::Response),
    /// Response to the `set_mac_poll_failure_wait_time` command.
    SetMacPollFailureWaitTime(set_mac_poll_failure_wait_time::Response),
    /// Response to the `set_multicast_table_entry` command.
    SetMulticastTableEntry(set_multicast_table_entry::Response),
    /// Response to the `set_source_route_discovery_mode` command.
    SetSourceRouteDiscoveryMode(set_source_route_discovery_mode::Response),
    /// Response to the `unicast_current_network_key` command.
    UnicastCurrentNetworkKey(unicast_current_network_key::Response),
    /// Response to the `write_node_data` command.
    WriteNodeData(write_node_data::Response),
}
