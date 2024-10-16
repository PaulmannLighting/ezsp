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

impl From<ChildJoin> for Handler {
    fn from(handler: ChildJoin) -> Self {
        Self::ChildJoin(handler)
    }
}

impl From<DutyCycle> for Handler {
    fn from(handler: DutyCycle) -> Self {
        Self::DutyCycle(handler)
    }
}

impl From<EnergyScanResult> for Handler {
    fn from(handler: EnergyScanResult) -> Self {
        Self::EnergyScanResult(handler)
    }
}

impl From<NetworkFound> for Handler {
    fn from(handler: NetworkFound) -> Self {
        Self::NetworkFound(handler)
    }
}

impl From<ScanComplete> for Handler {
    fn from(handler: ScanComplete) -> Self {
        Self::ScanComplete(handler)
    }
}

impl From<StackStatus> for Handler {
    fn from(handler: StackStatus) -> Self {
        Self::StackStatus(handler)
    }
}

impl From<UnusedPanIdFound> for Handler {
    fn from(handler: UnusedPanIdFound) -> Self {
        Self::UnusedPanIdFound(handler)
    }
}
