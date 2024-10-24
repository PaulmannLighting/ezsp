//! Networking event handlers.

mod child_join;
mod duty_cycle;
mod energy_scan_result;
mod network_found;
mod scan_complete;
mod stack_status;
mod unused_pan_id_found;

pub use child_join::Handler as ChildJoin;
pub use duty_cycle::Handler as DutyCycle;
pub use energy_scan_result::Handler as EnergyScanResult;
pub use network_found::Handler as NetworkFound;
pub use scan_complete::Handler as ScanComplete;
pub use stack_status::Handler as StackStatus;
pub use unused_pan_id_found::Handler as UnusedPanIdFound;

/// Handler of a networking event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Child join event.
    ChildJoin(ChildJoin),
    /// Duty cycle event.
    DutyCycle(DutyCycle),
    /// Energy scan result event.
    EnergyScanResult(EnergyScanResult),
    /// Network found event.
    NetworkFound(NetworkFound),
    /// Scan complete event.
    ScanComplete(ScanComplete),
    /// Stack status event.
    StackStatus(StackStatus),
    /// Unused PAN ID found event.
    UnusedPanIdFound(UnusedPanIdFound),
}
