//! Networking event handlers.

pub use self::child_join::Handler as ChildJoin;
pub use self::duty_cycle::Handler as DutyCycle;
pub use self::energy_scan_result::Handler as EnergyScanResult;
pub use self::network_found::Handler as NetworkFound;
pub use self::scan_complete::Handler as ScanComplete;
pub use self::stack_status::Handler as StackStatus;
pub use self::unused_pan_id_found::Handler as UnusedPanIdFound;

mod child_join;
mod duty_cycle;
mod energy_scan_result;
mod network_found;
mod scan_complete;
mod stack_status;
mod unused_pan_id_found;
crate::frame::parameters::parameter_enum!(
    Handler,
    ChildJoin,
    DutyCycle,
    EnergyScanResult,
    NetworkFound,
    ScanComplete,
    StackStatus,
    UnusedPanIdFound
);
