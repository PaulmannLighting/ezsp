//! Networking Frames

pub(crate) mod child_id;
pub(crate) mod clear_stored_beacons;
pub(crate) mod energy_scan_request;
pub(crate) mod find_and_rejoin_network;
pub(crate) mod find_unused_pan_id;
pub(crate) mod form_network;
pub(crate) mod get_child_data;
pub(crate) mod get_current_duty_cycle;
pub(crate) mod get_duty_cycle_limits;
pub(crate) mod get_duty_cycle_state;
pub(crate) mod get_first_beacon;
pub(crate) mod get_logical_channel;
pub(crate) mod get_neighbor;
pub(crate) mod get_neighbor_frame_counter;
pub(crate) mod get_network_parameters;
pub(crate) mod get_next_beacon;
pub(crate) mod get_num_stored_beacons;
pub mod get_parent_child_parameters;
pub(crate) mod get_radio_channel;
pub(crate) mod get_radio_parameters;
pub(crate) mod get_route_table_entry;
pub(crate) mod get_routing_shortcut_threshold;
pub mod get_source_route_table_entry;
pub(crate) mod get_source_route_table_filled_size;
pub(crate) mod get_source_route_table_total_size;
pub mod handler;
pub(crate) mod id;
pub(crate) mod join_network;
pub(crate) mod join_network_directly;
pub(crate) mod leave_network;
pub(crate) mod multi_phy_set_radio_channel;
pub(crate) mod multi_phy_set_radio_power;
pub(crate) mod multi_phy_start;
pub(crate) mod multi_phy_stop;
pub(crate) mod neighbor_count;
pub(crate) mod network_init;
pub(crate) mod network_state;
pub(crate) mod permit_joining;
pub(crate) mod send_link_power_delta_request;
pub(crate) mod set_broken_route_error_code;
pub(crate) mod set_child_data;
pub(crate) mod set_concentrator;
pub(crate) mod set_duty_cycle_limits_in_stack;
pub(crate) mod set_logical_and_radio_channel;
pub(crate) mod set_manufacturer_code;
pub(crate) mod set_neighbor_frame_counter;
pub(crate) mod set_power_descriptor;
pub(crate) mod set_radio_channel;
pub(crate) mod set_radio_ieee802154_cca_mode;
pub(crate) mod set_radio_power;
pub(crate) mod set_routing_shortcut_threshold;
pub(crate) mod start_scan;
pub(crate) mod stop_scan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    ChildId(child_id::Command),
    ClearStoredBeacons(clear_stored_beacons::Command),
    EnergyScanRequest(energy_scan_request::Command),
    FindAndRejoinNetwork(find_and_rejoin_network::Command),
    FindUnusedPanId(find_unused_pan_id::Command),
    FormNetwork(form_network::Command),
    GetChildData(get_child_data::Command),
    GetCurrentDutyCycle(get_current_duty_cycle::Command),
    GetDutyCycleLimits(get_duty_cycle_limits::Command),
    GetDutyCycleState(get_duty_cycle_state::Command),
    GetFirstBeacon(get_first_beacon::Command),
    GetLogicalChannel(get_logical_channel::Command),
    GetNeighbor(get_neighbor::Command),
    GetNeighborFrameCounter(get_neighbor_frame_counter::Command),
    GetNetworkParameters(get_network_parameters::Command),
    GetNextBeacon(get_next_beacon::Command),
    GetNumStoredBeacons(get_num_stored_beacons::Command),
    GetParentChildParameters(get_parent_child_parameters::Command),
    GetRadioChannel(get_radio_channel::Command),
    GetRadioParameters(get_radio_parameters::Command),
    GetRouteTableEntry(get_route_table_entry::Command),
    GetRoutingShortcutThreshold(get_routing_shortcut_threshold::Command),
    GetSourceRouteTableEntry(get_source_route_table_entry::Command),
    GetSourceRouteTableFilledSize(get_source_route_table_filled_size::Command),
    GetSourceRouteTableTotalSize(get_source_route_table_total_size::Command),
    Id(id::Command),
    JoinNetwork(join_network::Command),
    JoinNetworkDirectly(join_network_directly::Command),
    LeaveNetwork(leave_network::Command),
    MultiPhySetRadioChannel(multi_phy_set_radio_channel::Command),
    MultiPhySetRadioPower(multi_phy_set_radio_power::Command),
    MultiPhyStart(multi_phy_start::Command),
    MultiPhyStop(multi_phy_stop::Command),
    NeighborCount(neighbor_count::Command),
    NetworkInit(network_init::Command),
    NetworkState(network_state::Command),
    PermitJoining(permit_joining::Command),
    SendLinkPowerDeltaRequest(send_link_power_delta_request::Command),
    SetBrokenRouteErrorCode(set_broken_route_error_code::Command),
    SetChildData(set_child_data::Command),
    SetConcentrator(set_concentrator::Command),
    SetDutyCycleLimitsInStack(set_duty_cycle_limits_in_stack::Command),
    SetLogicalAndRadioChannel(set_logical_and_radio_channel::Command),
    SetManufacturerCode(set_manufacturer_code::Command),
    SetNeighborFrameCounter(set_neighbor_frame_counter::Command),
    SetPowerDescriptor(set_power_descriptor::Command),
    SetRadioChannel(set_radio_channel::Command),
    SetRadioIeee802154CcaMode(set_radio_ieee802154_cca_mode::Command),
    SetRadioPower(set_radio_power::Command),
    SetRoutingShortcutThreshold(set_routing_shortcut_threshold::Command),
    StartScan(start_scan::Command),
    StopScan(stop_scan::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    ChildId(child_id::Response),
    ClearStoredBeacons(clear_stored_beacons::Response),
    EnergyScanRequest(energy_scan_request::Response),
    FindAndRejoinNetwork(find_and_rejoin_network::Response),
    FindUnusedPanId(find_unused_pan_id::Response),
    FormNetwork(form_network::Response),
    GetChildData(get_child_data::Response),
    GetCurrentDutyCycle(get_current_duty_cycle::Response),
    GetDutyCycleLimits(get_duty_cycle_limits::Response),
    GetDutyCycleState(get_duty_cycle_state::Response),
    GetFirstBeacon(get_first_beacon::Response),
    GetLogicalChannel(get_logical_channel::Response),
    GetNeighbor(get_neighbor::Response),
    GetNeighborFrameCounter(get_neighbor_frame_counter::Response),
    GetNetworkParameters(get_network_parameters::Response),
    GetNextBeacon(get_next_beacon::Response),
    GetNumStoredBeacons(get_num_stored_beacons::Response),
    GetParentChildParameters(get_parent_child_parameters::Response),
    GetRadioChannel(get_radio_channel::Response),
    GetRadioParameters(get_radio_parameters::Response),
    GetRouteTableEntry(get_route_table_entry::Response),
    GetRoutingShortcutThreshold(get_routing_shortcut_threshold::Response),
    GetSourceRouteTableEntry(get_source_route_table_entry::Response),
    GetSourceRouteTableFilledSize(get_source_route_table_filled_size::Response),
    GetSourceRouteTableTotalSize(get_source_route_table_total_size::Response),
    Id(id::Response),
    JoinNetwork(join_network::Response),
    JoinNetworkDirectly(join_network_directly::Response),
    LeaveNetwork(leave_network::Response),
    MultiPhySetRadioChannel(multi_phy_set_radio_channel::Response),
    MultiPhySetRadioPower(multi_phy_set_radio_power::Response),
    MultiPhyStart(multi_phy_start::Response),
    MultiPhyStop(multi_phy_stop::Response),
    NeighborCount(neighbor_count::Response),
    NetworkInit(network_init::Response),
    NetworkState(network_state::Response),
    PermitJoining(permit_joining::Response),
    SendLinkPowerDeltaRequest(send_link_power_delta_request::Response),
    SetBrokenRouteErrorCode(set_broken_route_error_code::Response),
    SetChildData(set_child_data::Response),
    SetConcentrator(set_concentrator::Response),
    SetDutyCycleLimitsInStack(set_duty_cycle_limits_in_stack::Response),
    SetLogicalAndRadioChannel(set_logical_and_radio_channel::Response),
    SetManufacturerCode(set_manufacturer_code::Response),
    SetNeighborFrameCounter(set_neighbor_frame_counter::Response),
    SetPowerDescriptor(set_power_descriptor::Response),
    SetRadioChannel(set_radio_channel::Response),
    SetRadioIeee802154CcaMode(set_radio_ieee802154_cca_mode::Response),
    SetRadioPower(set_radio_power::Response),
    SetRoutingShortcutThreshold(set_routing_shortcut_threshold::Response),
    StartScan(start_scan::Response),
    StopScan(stop_scan::Response),
    Handler(handler::Handler),
}
