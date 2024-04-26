pub mod child_join;
pub mod duty_cycle;
pub mod energy_scan_result;
pub mod network_found;
pub mod scan_complete;
pub mod stack_status;
pub mod unused_pan_id_found;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    ChildJoin(child_join::Handler),
    DutyCycle(duty_cycle::Handler),
    EnergyScan(energy_scan_result::Handler),
    NetworkFound(network_found::Handler),
    ScanComplete(scan_complete::Handler),
    StackStatus(stack_status::Handler),
    UnusedPanIdFound(unused_pan_id_found::Handler),
}
