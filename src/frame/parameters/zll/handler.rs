//! Handlers for the ZLL commands.

pub use self::address_assignment::Handler as AddressAssignment;
pub use self::network_found::Handler as NetworkFound;
pub use self::scan_complete::Handler as ScanComplete;
pub use self::touch_link_target::Handler as TouchLinkTarget;

mod address_assignment;
mod network_found;
mod scan_complete;
mod touch_link_target;

/// The handler for the ZLL commands.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the address assignment command.
    AddressAssignment(AddressAssignment),
    /// The handler for the network found command.
    NetworkFound(NetworkFound),
    /// The handler for the scan complete command.
    ScanComplete(ScanComplete),
    /// The handler for the touch link target command.
    TouchLinkTarget(TouchLinkTarget),
}
