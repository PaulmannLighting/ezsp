//! Handlers for the ZLL commands.

mod address_assignment;
mod network_found;
mod scan_complete;
mod touch_link_target;

pub use address_assignment::Handler as AddressAssignment;
pub use network_found::Handler as NetworkFound;
pub use scan_complete::Handler as ScanComplete;
pub use touch_link_target::Handler as TouchLinkTarget;

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

impl From<AddressAssignment> for Handler {
    fn from(handler: AddressAssignment) -> Self {
        Self::AddressAssignment(handler)
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

impl From<TouchLinkTarget> for Handler {
    fn from(handler: TouchLinkTarget) -> Self {
        Self::TouchLinkTarget(handler)
    }
}
