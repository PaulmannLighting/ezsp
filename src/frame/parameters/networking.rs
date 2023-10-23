use crate::read_write::Writable;
use std::io::Write;

pub mod child_join_handler;
pub mod energy_scan_request;
pub mod energy_scan_result_handler;
pub mod find_and_rejoin_network;
pub mod find_unused_pan_id;
pub mod form_network;
pub mod get_child_data;
pub mod get_network_parameters;
pub mod get_parent_child_parameters;
pub mod get_radio_parameters;
pub mod join_network;
pub mod join_network_directly;
pub mod leave_network;
pub mod network_found_handler;
pub mod network_init;
pub mod network_state;
pub mod permit_joining;
pub mod scan_complete_handler;
pub mod set_manufacturer_code;
pub mod set_power_descriptor;
pub mod stack_status_handler;
pub mod start_scan;
pub mod stop_scan;
pub mod unused_pan_id_found_handler;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    EnergyScanRequest(energy_scan_request::Command),
    FindAndRejoinNetwork(find_and_rejoin_network::Command),
    FindUnusedPanId(find_unused_pan_id::Command),
    FormNetwork(form_network::Command),
    GetChildData(get_child_data::Command),
    GetNetworkParameters(get_network_parameters::Command),
    GetParentChildParameters(get_parent_child_parameters::Command),
    GetRadioParameters(get_radio_parameters::Command),
    JoinNetwork(join_network::Command),
    JoinNetworkDirectly(join_network_directly::Command),
    LeaveNetwork(leave_network::Command),
    NetworkInit(network_init::Command),
    NetworkState(network_state::Command),
    PermitJoining(permit_joining::Command),
    SetManufacturerCode(set_manufacturer_code::Command),
    SetPowerDescriptor(set_power_descriptor::Command),
    StartScan(start_scan::Command),
    StopScan(stop_scan::Command),
}

impl Command {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::EnergyScanRequest(_) => energy_scan_request::ID,
            Self::FindAndRejoinNetwork(_) => find_and_rejoin_network::ID,
            Self::FindUnusedPanId(_) => find_unused_pan_id::ID,
            Self::FormNetwork(_) => form_network::ID,
            Self::GetChildData(_) => get_child_data::ID,
            Self::GetNetworkParameters(_) => get_network_parameters::ID,
            Self::GetParentChildParameters(_) => get_parent_child_parameters::ID,
            Self::GetRadioParameters(_) => get_radio_parameters::ID,
            Self::JoinNetwork(_) => join_network::ID,
            Self::JoinNetworkDirectly(_) => join_network_directly::ID,
            Self::LeaveNetwork(_) => leave_network::ID,
            Self::NetworkInit(_) => network_init::ID,
            Self::NetworkState(_) => network_state::ID,
            Self::PermitJoining(_) => permit_joining::ID,
            Self::SetManufacturerCode(_) => set_manufacturer_code::ID,
            Self::SetPowerDescriptor(_) => set_power_descriptor::ID,
            Self::StartScan(_) => start_scan::ID,
            Self::StopScan(_) => stop_scan::ID,
        }
    }
}

impl Writable for Command {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::EnergyScanRequest(energy_scan_request) => energy_scan_request.write_to(dst),
            Self::FindAndRejoinNetwork(find_and_rejoin_network) => {
                find_and_rejoin_network.write_to(dst)
            }
            Self::FindUnusedPanId(find_unused_pan_id) => find_unused_pan_id.write_to(dst),
            Self::FormNetwork(form_network) => form_network.write_to(dst),
            Self::GetChildData(get_child_data) => get_child_data.write_to(dst),
            Self::GetNetworkParameters(get_network_parameters) => {
                get_network_parameters.write_to(dst)
            }
            Self::GetParentChildParameters(get_parent_child_parameters) => {
                get_parent_child_parameters.write_to(dst)
            }
            Self::GetRadioParameters(get_radio_parameters) => get_radio_parameters.write_to(dst),
            Self::JoinNetwork(join_network) => join_network.write_to(dst),
            Self::JoinNetworkDirectly(join_network_directly) => join_network_directly.write_to(dst),
            Self::LeaveNetwork(leave_network) => leave_network.write_to(dst),
            Self::NetworkInit(network_init) => network_init.write_to(dst),
            Self::NetworkState(network_state) => network_state.write_to(dst),
            Self::PermitJoining(permit_joining) => permit_joining.write_to(dst),
            Self::SetManufacturerCode(set_manufacturer_code) => set_manufacturer_code.write_to(dst),
            Self::SetPowerDescriptor(set_power_descriptor) => set_power_descriptor.write_to(dst),
            Self::StartScan(start_scan) => start_scan.write_to(dst),
            Self::StopScan(stop_scan) => stop_scan.write_to(dst),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Response {
    EnergyScanRequest(energy_scan_request::Response),
    FindAndRejoinNetwork(find_and_rejoin_network::Response),
    FindUnusedPanId(find_unused_pan_id::Response),
    FormNetwork(form_network::Response),
    GetChildData(get_child_data::Response),
    GetNetworkParameters(get_network_parameters::Response),
    GetParentChildParameters(get_parent_child_parameters::Response),
    GetRadioParameters(get_radio_parameters::Response),
    JoinNetwork(join_network::Response),
    JoinNetworkDirectly(join_network_directly::Response),
    LeaveNetwork(leave_network::Response),
    NetworkInit(network_init::Response),
    NetworkState(network_state::Response),
    PermitJoining(permit_joining::Response),
    SetManufacturerCode(set_manufacturer_code::Response),
    SetPowerDescriptor(set_power_descriptor::Response),
    StartScan(start_scan::Response),
    StopScan(stop_scan::Response),
}

impl Response {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::EnergyScanRequest(_) => energy_scan_request::ID,
            Self::FindAndRejoinNetwork(_) => find_and_rejoin_network::ID,
            Self::FindUnusedPanId(_) => find_unused_pan_id::ID,
            Self::FormNetwork(_) => form_network::ID,
            Self::GetChildData(_) => get_child_data::ID,
            Self::GetNetworkParameters(_) => get_network_parameters::ID,
            Self::GetParentChildParameters(_) => get_parent_child_parameters::ID,
            Self::GetRadioParameters(_) => get_radio_parameters::ID,
            Self::JoinNetwork(_) => join_network::ID,
            Self::JoinNetworkDirectly(_) => join_network_directly::ID,
            Self::LeaveNetwork(_) => leave_network::ID,
            Self::NetworkInit(_) => network_init::ID,
            Self::NetworkState(_) => network_state::ID,
            Self::PermitJoining(_) => permit_joining::ID,
            Self::SetManufacturerCode(_) => set_manufacturer_code::ID,
            Self::SetPowerDescriptor(_) => set_power_descriptor::ID,
            Self::StartScan(_) => start_scan::ID,
            Self::StopScan(_) => stop_scan::ID,
        }
    }
}

impl Writable for Response {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::EnergyScanRequest(energy_scan_request) => energy_scan_request.write_to(dst),
            Self::FindAndRejoinNetwork(find_and_rejoin_network) => {
                find_and_rejoin_network.write_to(dst)
            }
            Self::FindUnusedPanId(find_unused_pan_id) => find_unused_pan_id.write_to(dst),
            Self::FormNetwork(form_network) => form_network.write_to(dst),
            Self::GetChildData(get_child_data) => get_child_data.write_to(dst),
            Self::GetNetworkParameters(get_network_parameters) => {
                get_network_parameters.write_to(dst)
            }
            Self::GetParentChildParameters(get_parent_child_parameters) => {
                get_parent_child_parameters.write_to(dst)
            }
            Self::GetRadioParameters(get_radio_parameters) => get_radio_parameters.write_to(dst),
            Self::JoinNetwork(join_network) => join_network.write_to(dst),
            Self::JoinNetworkDirectly(join_network_directly) => join_network_directly.write_to(dst),
            Self::LeaveNetwork(leave_network) => leave_network.write_to(dst),
            Self::NetworkInit(network_init) => network_init.write_to(dst),
            Self::NetworkState(network_state) => network_state.write_to(dst),
            Self::PermitJoining(permit_joining) => permit_joining.write_to(dst),
            Self::SetManufacturerCode(set_manufacturer_code) => set_manufacturer_code.write_to(dst),
            Self::SetPowerDescriptor(set_power_descriptor) => set_power_descriptor.write_to(dst),
            Self::StartScan(start_scan) => start_scan.write_to(dst),
            Self::StopScan(stop_scan) => stop_scan.write_to(dst),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Callback {
    ChildJoin(child_join_handler::Response),
    EnergyScanResult(energy_scan_result_handler::Response),
    NetworkFound(network_found_handler::Response),
    ScanComplete(scan_complete_handler::Response),
    StackStatus(stack_status_handler::Response),
    UnusedPanIdFound(unused_pan_id_found_handler::Response),
}

impl Callback {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::ChildJoin(_) => child_join_handler::ID,
            Self::EnergyScanResult(_) => energy_scan_result_handler::ID,
            Self::NetworkFound(_) => network_found_handler::ID,
            Self::ScanComplete(_) => scan_complete_handler::ID,
            Self::StackStatus(_) => stack_status_handler::ID,
            Self::UnusedPanIdFound(_) => unused_pan_id_found_handler::ID,
        }
    }
}

impl Writable for Callback {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::ChildJoin(child_join_handler) => child_join_handler.write_to(dst),
            Self::EnergyScanResult(energy_scan_result_handler) => {
                energy_scan_result_handler.write_to(dst)
            }
            Self::NetworkFound(network_found_handler) => network_found_handler.write_to(dst),
            Self::ScanComplete(scan_complete_handler) => scan_complete_handler.write_to(dst),
            Self::StackStatus(stack_status_handler) => stack_status_handler.write_to(dst),
            Self::UnusedPanIdFound(unused_pan_id_found_handler) => {
                unused_pan_id_found_handler.write_to(dst)
            }
        }
    }
}
