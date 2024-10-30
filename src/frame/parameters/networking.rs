//! Networking frames.

pub mod child_id;
pub mod clear_stored_beacons;
pub mod energy_scan_request;
pub mod find_and_rejoin_network;
pub mod find_unused_pan_id;
pub mod form_network;
pub mod get_child_data;
pub mod get_current_duty_cycle;
pub mod get_duty_cycle_limits;
pub mod get_duty_cycle_state;
pub mod get_first_beacon;
pub mod get_logical_channel;
pub mod get_neighbor;
pub mod get_neighbor_frame_counter;
pub mod get_network_parameters;
pub mod get_next_beacon;
pub mod get_num_stored_beacons;
pub mod get_parent_child_parameters;
pub mod get_radio_channel;
pub mod get_radio_parameters;
pub mod get_route_table_entry;
pub mod get_routing_shortcut_threshold;
pub mod get_source_route_table_entry;
pub mod get_source_route_table_filled_size;
pub mod get_source_route_table_total_size;
pub mod handler;
pub mod id;
pub mod join_network;
pub mod join_network_directly;
pub mod leave_network;
pub mod multi_phy_set_radio_channel;
pub mod multi_phy_set_radio_power;
pub mod multi_phy_start;
pub mod multi_phy_stop;
pub mod neighbor_count;
pub mod network_init;
pub mod network_state;
pub mod permit_joining;
pub mod send_link_power_delta_request;
pub mod set_broken_route_error_code;
pub mod set_child_data;
pub mod set_concentrator;
pub mod set_duty_cycle_limits_in_stack;
pub mod set_logical_and_radio_channel;
pub mod set_manufacturer_code;
pub mod set_neighbor_frame_counter;
pub mod set_power_descriptor;
pub mod set_radio_channel;
pub mod set_radio_ieee802154_cca_mode;
pub mod set_radio_power;
pub mod set_routing_shortcut_threshold;
pub mod start_scan;
pub mod stop_scan;

/// Response parameters for networking commands.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response parameter for the `child_id` command.
    ChildId(child_id::Response),
    /// Response parameter for the `clear_stored_beacons` command.
    ClearStoredBeacons(clear_stored_beacons::Response),
    /// Response parameter for the `energy_scan_request` command.
    EnergyScanRequest(energy_scan_request::Response),
    /// Response parameter for the `find_and_rejoin_network` command.
    FindAndRejoinNetwork(find_and_rejoin_network::Response),
    /// Response parameter for the `find_unused_pan_id` command.
    FindUnusedPanId(find_unused_pan_id::Response),
    /// Response parameter for the `form_network` command.
    FormNetwork(form_network::Response),
    /// Response parameter for the `get_child_data` command.
    GetChildData(get_child_data::Response),
    /// Response parameter for the `get_current_duty_cycle` command.
    GetCurrentDutyCycle(Box<get_current_duty_cycle::Response>),
    /// Response parameter for the `get_duty_cycle_limits` command.
    GetDutyCycleLimits(get_duty_cycle_limits::Response),
    /// Response parameter for the `get_duty_cycle_state` command.
    GetDutyCycleState(get_duty_cycle_state::Response),
    /// Response parameter for the `get_first_beacon` command.
    GetFirstBeacon(get_first_beacon::Response),
    /// Response parameter for the `get_logical_channel` command.
    GetLogicalChannel(get_logical_channel::Response),
    /// Response parameter for the `get_neighbor` command.
    GetNeighbor(get_neighbor::Response),
    /// Response parameter for the `get_neighbor_frame_counter` command.
    GetNeighborFrameCounter(get_neighbor_frame_counter::Response),
    /// Response parameter for the `get_network_parameters` command.
    GetNetworkParameters(get_network_parameters::Response),
    /// Response parameter for the `get_next_beacon` command.
    GetNextBeacon(get_next_beacon::Response),
    /// Response parameter for the `get_num_stored_beacons` command.
    GetNumStoredBeacons(get_num_stored_beacons::Response),
    /// Response parameter for the `get_parent_child_parameters` command.
    GetParentChildParameters(get_parent_child_parameters::Response),
    /// Response parameter for the `get_radio_channel` command.
    GetRadioChannel(get_radio_channel::Response),
    /// Response parameter for the `get_radio_parameters` command.
    GetRadioParameters(get_radio_parameters::Response),
    /// Response parameter for the `get_route_table_entry` command.
    GetRouteTableEntry(get_route_table_entry::Response),
    /// Response parameter for the `get_routing_shortcut_threshold` command.
    GetRoutingShortcutThreshold(get_routing_shortcut_threshold::Response),
    /// Response parameter for the `get_source_route_table_entry` command.
    GetSourceRouteTableEntry(get_source_route_table_entry::Response),
    /// Response parameter for the `get_source_route_table_filled_size` command.
    GetSourceRouteTableFilledSize(get_source_route_table_filled_size::Response),
    /// Response parameter for the `get_source_route_table_total_size` command.
    GetSourceRouteTableTotalSize(get_source_route_table_total_size::Response),
    /// Response parameter for the `id` command.
    Id(id::Response),
    /// Response parameter for the `join_network` command.
    JoinNetwork(join_network::Response),
    /// Response parameter for the `join_network_directly` command.
    JoinNetworkDirectly(join_network_directly::Response),
    /// Response parameter for the `leave_network` command.
    LeaveNetwork(leave_network::Response),
    /// Response parameter for the `multi_phy_set_radio_channel` command.
    MultiPhySetRadioChannel(multi_phy_set_radio_channel::Response),
    /// Response parameter for the `multi_phy_set_radio_power` command.
    MultiPhySetRadioPower(multi_phy_set_radio_power::Response),
    /// Response parameter for the `multi_phy_start` command.
    MultiPhyStart(multi_phy_start::Response),
    /// Response parameter for the `multi_phy_stop` command.
    MultiPhyStop(multi_phy_stop::Response),
    /// Response parameter for the `neighbor_count` command.
    NeighborCount(neighbor_count::Response),
    /// Response parameter for the `network_init` command.
    NetworkInit(network_init::Response),
    /// Response parameter for the `network_state` command.
    NetworkState(network_state::Response),
    /// Response parameter for the `permit_joining` command.
    PermitJoining(permit_joining::Response),
    /// Response parameter for the `send_link_power_delta_request` command.
    SendLinkPowerDeltaRequest(send_link_power_delta_request::Response),
    /// Response parameter for the `set_broken_route_error_code` command.
    SetBrokenRouteErrorCode(set_broken_route_error_code::Response),
    /// Response parameter for the `set_child_data` command.
    SetChildData(set_child_data::Response),
    /// Response parameter for the `set_concentrator` command.
    SetConcentrator(set_concentrator::Response),
    /// Response parameter for the `set_duty_cycle_limits_in_stack` command.
    SetDutyCycleLimitsInStack(set_duty_cycle_limits_in_stack::Response),
    /// Response parameter for the `set_logical_and_radio_channel` command.
    SetLogicalAndRadioChannel(set_logical_and_radio_channel::Response),
    /// Response parameter for the `set_manufacturer_code` command.
    SetManufacturerCode(set_manufacturer_code::Response),
    /// Response parameter for the `set_neighbor_frame_counter` command.
    SetNeighborFrameCounter(set_neighbor_frame_counter::Response),
    /// Response parameter for the `set_power_descriptor` command.
    SetPowerDescriptor(set_power_descriptor::Response),
    /// Response parameter for the `set_radio_channel` command.
    SetRadioChannel(set_radio_channel::Response),
    /// Response parameter for the `set_radio_ieee802154_cca_mode` command.
    SetRadioIeee802154CcaMode(set_radio_ieee802154_cca_mode::Response),
    /// Response parameter for the `set_radio_power` command.
    SetRadioPower(set_radio_power::Response),
    /// Response parameter for the `set_routing_shortcut_threshold` command.
    SetRoutingShortcutThreshold(set_routing_shortcut_threshold::Response),
    /// Response parameter for the `start_scan` command.
    StartScan(start_scan::Response),
    /// Response parameter for the `stop_scan` command.
    StopScan(stop_scan::Response),
}
