pub mod child_join;
pub mod duty_cycle;
pub mod energy_scan_result;
pub mod network_found;
pub mod scan_complete;
pub mod stack_status;
pub mod unused_pan_id_found;

/// Handler of a networking event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Child join event.
    ChildJoin(child_join::Handler),
    /// Duty cycle event.
    DutyCycle(duty_cycle::Handler),
    /// Energy scan result event.
    EnergyScan(energy_scan_result::Handler),
    /// Network found event.
    NetworkFound(network_found::Handler),
    /// Scan complete event.
    ScanComplete(scan_complete::Handler),
    /// Stack status event.
    StackStatus(stack_status::Handler),
    /// Unused PAN ID found event.
    UnusedPanIdFound(unused_pan_id_found::Handler),
}
