//! Networking Frames

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    ChildId(child_id::Response),
    ClearStoredBeacons(clear_stored_beacons::Response),
    EnergyScanRequest(energy_scan_request::Response),
    FindAndRejoinNetwork(find_and_rejoin_network::Response),
    FindUnusedPanId(find_unused_pan_id::Response),
    FormNetwork(form_network::Response),
    GetChildData(get_child_data::Response),
    GetCurrentDutyCycle(Box<get_current_duty_cycle::Response>),
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
