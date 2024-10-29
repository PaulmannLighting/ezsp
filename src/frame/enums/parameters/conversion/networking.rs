use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::networking;

impl TryFrom<Parameters> for networking::child_id::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::ChildId(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::clear_stored_beacons::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::ClearStoredBeacons(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::energy_scan_request::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::EnergyScanRequest(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::find_and_rejoin_network::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::FindAndRejoinNetwork(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::find_unused_pan_id::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::FindUnusedPanId(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::form_network::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::FormNetwork(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_child_data::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::GetChildData(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_current_duty_cycle::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetCurrentDutyCycle(response),
            )) => Ok(*response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_duty_cycle_limits::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetDutyCycleLimits(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_duty_cycle_state::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetDutyCycleState(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_first_beacon::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::GetFirstBeacon(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_logical_channel::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetLogicalChannel(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_neighbor::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::GetNeighbor(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_neighbor_frame_counter::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetNeighborFrameCounter(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_network_parameters::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetNetworkParameters(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_next_beacon::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::GetNextBeacon(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_num_stored_beacons::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetNumStoredBeacons(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_parent_child_parameters::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetParentChildParameters(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_radio_channel::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::GetRadioChannel(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_radio_parameters::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetRadioParameters(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_route_table_entry::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetRouteTableEntry(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_routing_shortcut_threshold::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetRoutingShortcutThreshold(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_source_route_table_entry::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetSourceRouteTableEntry(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_source_route_table_filled_size::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetSourceRouteTableFilledSize(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::get_source_route_table_total_size::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::GetSourceRouteTableTotalSize(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::id::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::Id(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::join_network::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::JoinNetwork(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::join_network_directly::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::JoinNetworkDirectly(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::leave_network::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::LeaveNetwork(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::multi_phy_set_radio_channel::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::MultiPhySetRadioChannel(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::multi_phy_set_radio_power::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(
                networking::Response::MultiPhySetRadioPower(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::multi_phy_start::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::MultiPhyStart(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::multi_phy_stop::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::MultiPhyStop(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::neighbor_count::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::NeighborCount(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for networking::start_scan::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::StartScan(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}
