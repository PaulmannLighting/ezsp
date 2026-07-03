//! Handlers for the ZLL commands.

pub use self::address_assignment::Handler as AddressAssignment;
pub use self::network_found::Handler as NetworkFound;
pub use self::scan_complete::Handler as ScanComplete;
pub use self::touch_link_target::Handler as TouchLinkTarget;

mod address_assignment;
mod network_found;
mod scan_complete;
mod touch_link_target;
crate::frame::parameters::parameter_enum!(
    Handler,
    AddressAssignment,
    NetworkFound,
    ScanComplete,
    TouchLinkTarget
);
