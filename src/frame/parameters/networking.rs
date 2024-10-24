//! Networking Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

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

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <child_id::Response as Parameter>::ID => Ok(Self::ChildId(
                child_id::Response::from_le_stream_exact(stream)?,
            )),
            <clear_stored_beacons::Response as Parameter>::ID => Ok(Self::ClearStoredBeacons(
                clear_stored_beacons::Response::from_le_stream_exact(stream)?,
            )),
            <energy_scan_request::Response as Parameter>::ID => Ok(Self::EnergyScanRequest(
                energy_scan_request::Response::from_le_stream_exact(stream)?,
            )),
            <find_and_rejoin_network::Response as Parameter>::ID => Ok(Self::FindAndRejoinNetwork(
                find_and_rejoin_network::Response::from_le_stream_exact(stream)?,
            )),
            <find_unused_pan_id::Response as Parameter>::ID => Ok(Self::FindUnusedPanId(
                find_unused_pan_id::Response::from_le_stream_exact(stream)?,
            )),
            <form_network::Response as Parameter>::ID => Ok(Self::FormNetwork(
                form_network::Response::from_le_stream_exact(stream)?,
            )),
            <get_child_data::Response as Parameter>::ID => Ok(Self::GetChildData(
                get_child_data::Response::from_le_stream_exact(stream)?,
            )),
            <get_current_duty_cycle::Response as Parameter>::ID => Ok(Self::GetCurrentDutyCycle(
                get_current_duty_cycle::Response::from_le_stream_exact(stream)?,
            )),
            <get_duty_cycle_limits::Response as Parameter>::ID => Ok(Self::GetDutyCycleLimits(
                get_duty_cycle_limits::Response::from_le_stream_exact(stream)?,
            )),
            <get_duty_cycle_state::Response as Parameter>::ID => Ok(Self::GetDutyCycleState(
                get_duty_cycle_state::Response::from_le_stream_exact(stream)?,
            )),
            <get_first_beacon::Response as Parameter>::ID => Ok(Self::GetFirstBeacon(
                get_first_beacon::Response::from_le_stream_exact(stream)?,
            )),
            <get_logical_channel::Response as Parameter>::ID => Ok(Self::GetLogicalChannel(
                get_logical_channel::Response::from_le_stream_exact(stream)?,
            )),
            <get_neighbor::Response as Parameter>::ID => Ok(Self::GetNeighbor(
                get_neighbor::Response::from_le_stream_exact(stream)?,
            )),
            <get_neighbor_frame_counter::Response as Parameter>::ID => {
                Ok(Self::GetNeighborFrameCounter(
                    get_neighbor_frame_counter::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_network_parameters::Response as Parameter>::ID => Ok(Self::GetNetworkParameters(
                get_network_parameters::Response::from_le_stream_exact(stream)?,
            )),
            <get_next_beacon::Response as Parameter>::ID => Ok(Self::GetNextBeacon(
                get_next_beacon::Response::from_le_stream_exact(stream)?,
            )),
            <get_num_stored_beacons::Response as Parameter>::ID => Ok(Self::GetNumStoredBeacons(
                get_num_stored_beacons::Response::from_le_stream_exact(stream)?,
            )),
            <get_parent_child_parameters::Response as Parameter>::ID => {
                Ok(Self::GetParentChildParameters(
                    get_parent_child_parameters::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_radio_channel::Response as Parameter>::ID => Ok(Self::GetRadioChannel(
                get_radio_channel::Response::from_le_stream_exact(stream)?,
            )),
            <get_radio_parameters::Response as Parameter>::ID => Ok(Self::GetRadioParameters(
                get_radio_parameters::Response::from_le_stream_exact(stream)?,
            )),
            <get_route_table_entry::Response as Parameter>::ID => Ok(Self::GetRouteTableEntry(
                get_route_table_entry::Response::from_le_stream_exact(stream)?,
            )),
            <get_routing_shortcut_threshold::Response as Parameter>::ID => {
                Ok(Self::GetRoutingShortcutThreshold(
                    get_routing_shortcut_threshold::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_source_route_table_entry::Response as Parameter>::ID => {
                Ok(Self::GetSourceRouteTableEntry(
                    get_source_route_table_entry::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_source_route_table_filled_size::Response as Parameter>::ID => {
                Ok(Self::GetSourceRouteTableFilledSize(
                    get_source_route_table_filled_size::Response::from_le_stream_exact(stream)?,
                ))
            }
            <get_source_route_table_total_size::Response as Parameter>::ID => {
                Ok(Self::GetSourceRouteTableTotalSize(
                    get_source_route_table_total_size::Response::from_le_stream_exact(stream)?,
                ))
            }
            <id::Response as Parameter>::ID => {
                Ok(Self::Id(id::Response::from_le_stream_exact(stream)?))
            }
            <join_network::Response as Parameter>::ID => Ok(Self::JoinNetwork(
                join_network::Response::from_le_stream_exact(stream)?,
            )),
            <join_network_directly::Response as Parameter>::ID => Ok(Self::JoinNetworkDirectly(
                join_network_directly::Response::from_le_stream_exact(stream)?,
            )),
            <leave_network::Response as Parameter>::ID => Ok(Self::LeaveNetwork(
                leave_network::Response::from_le_stream_exact(stream)?,
            )),
            <multi_phy_set_radio_channel::Response as Parameter>::ID => {
                Ok(Self::MultiPhySetRadioChannel(
                    multi_phy_set_radio_channel::Response::from_le_stream_exact(stream)?,
                ))
            }
            <multi_phy_set_radio_power::Response as Parameter>::ID => {
                Ok(Self::MultiPhySetRadioPower(
                    multi_phy_set_radio_power::Response::from_le_stream_exact(stream)?,
                ))
            }
            <multi_phy_start::Response as Parameter>::ID => Ok(Self::MultiPhyStart(
                multi_phy_start::Response::from_le_stream_exact(stream)?,
            )),
            <multi_phy_stop::Response as Parameter>::ID => Ok(Self::MultiPhyStop(
                multi_phy_stop::Response::from_le_stream_exact(stream)?,
            )),
            <neighbor_count::Response as Parameter>::ID => Ok(Self::NeighborCount(
                neighbor_count::Response::from_le_stream_exact(stream)?,
            )),
            <network_init::Response as Parameter>::ID => Ok(Self::NetworkInit(
                network_init::Response::from_le_stream_exact(stream)?,
            )),
            <network_state::Response as Parameter>::ID => Ok(Self::NetworkState(
                network_state::Response::from_le_stream_exact(stream)?,
            )),
            <permit_joining::Response as Parameter>::ID => Ok(Self::PermitJoining(
                permit_joining::Response::from_le_stream_exact(stream)?,
            )),
            <send_link_power_delta_request::Response as Parameter>::ID => {
                Ok(Self::SendLinkPowerDeltaRequest(
                    send_link_power_delta_request::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_broken_route_error_code::Response as Parameter>::ID => {
                Ok(Self::SetBrokenRouteErrorCode(
                    set_broken_route_error_code::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_child_data::Response as Parameter>::ID => Ok(Self::SetChildData(
                set_child_data::Response::from_le_stream_exact(stream)?,
            )),
            <set_concentrator::Response as Parameter>::ID => Ok(Self::SetConcentrator(
                set_concentrator::Response::from_le_stream_exact(stream)?,
            )),
            <set_duty_cycle_limits_in_stack::Response as Parameter>::ID => {
                Ok(Self::SetDutyCycleLimitsInStack(
                    set_duty_cycle_limits_in_stack::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_logical_and_radio_channel::Response as Parameter>::ID => {
                Ok(Self::SetLogicalAndRadioChannel(
                    set_logical_and_radio_channel::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_manufacturer_code::Response as Parameter>::ID => Ok(Self::SetManufacturerCode(
                set_manufacturer_code::Response::from_le_stream_exact(stream)?,
            )),
            <set_neighbor_frame_counter::Response as Parameter>::ID => {
                Ok(Self::SetNeighborFrameCounter(
                    set_neighbor_frame_counter::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_power_descriptor::Response as Parameter>::ID => Ok(Self::SetPowerDescriptor(
                set_power_descriptor::Response::from_le_stream_exact(stream)?,
            )),
            <set_radio_channel::Response as Parameter>::ID => Ok(Self::SetRadioChannel(
                set_radio_channel::Response::from_le_stream_exact(stream)?,
            )),
            <set_radio_ieee802154_cca_mode::Response as Parameter>::ID => {
                Ok(Self::SetRadioIeee802154CcaMode(
                    set_radio_ieee802154_cca_mode::Response::from_le_stream_exact(stream)?,
                ))
            }
            <set_radio_power::Response as Parameter>::ID => Ok(Self::SetRadioPower(
                set_radio_power::Response::from_le_stream_exact(stream)?,
            )),
            <set_routing_shortcut_threshold::Response as Parameter>::ID => {
                Ok(Self::SetRoutingShortcutThreshold(
                    set_routing_shortcut_threshold::Response::from_le_stream_exact(stream)?,
                ))
            }
            <start_scan::Response as Parameter>::ID => Ok(Self::StartScan(
                start_scan::Response::from_le_stream_exact(stream)?,
            )),
            <stop_scan::Response as Parameter>::ID => Ok(Self::StopScan(
                stop_scan::Response::from_le_stream_exact(stream)?,
            )),
            <handler::ChildJoin as Parameter>::ID => Ok(Self::Handler(
                handler::ChildJoin::from_le_stream_exact(stream)?.into(),
            )),
            <handler::DutyCycle as Parameter>::ID => Ok(Self::Handler(
                handler::DutyCycle::from_le_stream_exact(stream)?.into(),
            )),
            <handler::EnergyScanResult as Parameter>::ID => Ok(Self::Handler(
                handler::EnergyScanResult::from_le_stream_exact(stream)?.into(),
            )),
            <handler::NetworkFound as Parameter>::ID => Ok(Self::Handler(
                handler::NetworkFound::from_le_stream_exact(stream)?.into(),
            )),
            <handler::ScanComplete as Parameter>::ID => Ok(Self::Handler(
                handler::ScanComplete::from_le_stream_exact(stream)?.into(),
            )),
            <handler::StackStatus as Parameter>::ID => Ok(Self::Handler(
                handler::StackStatus::from_le_stream_exact(stream)?.into(),
            )),
            <handler::UnusedPanIdFound as Parameter>::ID => Ok(Self::Handler(
                handler::UnusedPanIdFound::from_le_stream_exact(stream)?.into(),
            )),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
