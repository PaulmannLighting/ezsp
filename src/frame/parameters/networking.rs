//! Networking frames.

pub use self::child_id::Response as ChildId;
pub use self::clear_stored_beacons::Response as ClearStoredBeacons;
pub use self::energy_scan_request::Response as EnergyScanRequest;
pub use self::find_and_rejoin_network::Response as FindAndRejoinNetwork;
pub use self::find_unused_pan_id::Response as FindUnusedPanId;
pub use self::form_network::Response as FormNetwork;
pub use self::get_child_data::Response as GetChildData;
pub use self::get_current_duty_cycle::Response as GetCurrentDutyCycle;
pub use self::get_duty_cycle_limits::Response as GetDutyCycleLimits;
pub use self::get_duty_cycle_state::Response as GetDutyCycleState;
pub use self::get_first_beacon::Response as GetFirstBeacon;
pub use self::get_logical_channel::Response as GetLogicalChannel;
pub use self::get_neighbor::Response as GetNeighbor;
pub use self::get_neighbor_frame_counter::Response as GetNeighborFrameCounter;
pub use self::get_network_parameters::Response as GetNetworkParameters;
pub use self::get_next_beacon::Response as GetNextBeacon;
pub use self::get_num_stored_beacons::Response as GetNumStoredBeacons;
pub use self::get_parent_child_parameters::Response as GetParentChildParameters;
pub use self::get_radio_channel::Response as GetRadioChannel;
pub use self::get_radio_parameters::Response as GetRadioParameters;
pub use self::get_route_table_entry::Response as GetRouteTableEntry;
pub use self::get_routing_shortcut_threshold::Response as GetRoutingShortcutThreshold;
pub use self::get_source_route_table_entry::Response as GetSourceRouteTableEntry;
pub use self::get_source_route_table_filled_size::Response as GetSourceRouteTableFilledSize;
pub use self::get_source_route_table_total_size::Response as GetSourceRouteTableTotalSize;
pub use self::id::Response as Id;
pub use self::join_network::Response as JoinNetwork;
pub use self::join_network_directly::Response as JoinNetworkDirectly;
pub use self::leave_network::Response as LeaveNetwork;
pub use self::multi_phy_set_radio_channel::Response as MultiPhySetRadioChannel;
pub use self::multi_phy_set_radio_power::Response as MultiPhySetRadioPower;
pub use self::multi_phy_start::Response as MultiPhyStart;
pub use self::multi_phy_stop::Response as MultiPhyStop;
pub use self::neighbor_count::Response as NeighborCount;
pub use self::network_init::Response as NetworkInit;
pub use self::network_state::Response as NetworkState;
pub use self::permit_joining::Response as PermitJoining;
pub use self::send_link_power_delta_request::Response as SendLinkPowerDeltaRequest;
pub use self::set_broken_route_error_code::Response as SetBrokenRouteErrorCode;
pub use self::set_child_data::Response as SetChildData;
pub use self::set_concentrator::Response as SetConcentrator;
pub use self::set_duty_cycle_limits_in_stack::Response as SetDutyCycleLimitsInStack;
pub use self::set_logical_and_radio_channel::Response as SetLogicalAndRadioChannel;
pub use self::set_manufacturer_code::Response as SetManufacturerCode;
pub use self::set_neighbor_frame_counter::Response as SetNeighborFrameCounter;
pub use self::set_power_descriptor::Response as SetPowerDescriptor;
pub use self::set_radio_channel::Response as SetRadioChannel;
pub use self::set_radio_ieee802154_cca_mode::Response as SetRadioIeee802154CcaMode;
pub use self::set_radio_power::Response as SetRadioPower;
pub use self::set_routing_shortcut_threshold::Response as SetRoutingShortcutThreshold;
pub use self::start_scan::Response as StartScan;
pub use self::stop_scan::Response as StopScan;

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

crate::frame::parameters::command_enum!(
    Command,
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
);

crate::frame::parameters::parameter_enum!(
    Response,
    ChildId,
    ClearStoredBeacons,
    EnergyScanRequest,
    FindAndRejoinNetwork,
    FindUnusedPanId,
    FormNetwork,
    GetChildData,
    GetCurrentDutyCycle,
    GetDutyCycleLimits,
    GetDutyCycleState,
    GetFirstBeacon,
    GetLogicalChannel,
    GetNeighbor,
    GetNeighborFrameCounter,
    GetNetworkParameters,
    GetNextBeacon,
    GetNumStoredBeacons,
    GetParentChildParameters,
    GetRadioChannel,
    GetRadioParameters,
    GetRouteTableEntry,
    GetRoutingShortcutThreshold,
    GetSourceRouteTableEntry,
    GetSourceRouteTableFilledSize,
    GetSourceRouteTableTotalSize,
    Id,
    JoinNetwork,
    JoinNetworkDirectly,
    LeaveNetwork,
    MultiPhySetRadioChannel,
    MultiPhySetRadioPower,
    MultiPhyStart,
    MultiPhyStop,
    NeighborCount,
    NetworkInit,
    NetworkState,
    PermitJoining,
    SendLinkPowerDeltaRequest,
    SetBrokenRouteErrorCode,
    SetChildData,
    SetConcentrator,
    SetDutyCycleLimitsInStack,
    SetLogicalAndRadioChannel,
    SetManufacturerCode,
    SetNeighborFrameCounter,
    SetPowerDescriptor,
    SetRadioChannel,
    SetRadioIeee802154CcaMode,
    SetRadioPower,
    SetRoutingShortcutThreshold,
    StartScan,
    StopScan
);
