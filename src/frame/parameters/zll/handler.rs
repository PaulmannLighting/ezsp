pub mod address_assignment;
pub mod network_found;
pub mod scan_complete;
pub mod touch_link_target;

/// The handler for the ZLL commands.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// The handler for the address assignment command.
    AddressAssignment(address_assignment::Handler),
    /// The handler for the network found command.
    NetworkFound(network_found::Handler),
    /// The handler for the scan complete command.
    ScanComplete(scan_complete::Handler),
    /// The handler for the touch link target command.
    TouchLinkTarget(touch_link_target::Handler),
}

impl From<address_assignment::Handler> for Handler {
    fn from(handler: address_assignment::Handler) -> Self {
        Self::AddressAssignment(handler)
    }
}

impl From<network_found::Handler> for Handler {
    fn from(handler: network_found::Handler) -> Self {
        Self::NetworkFound(handler)
    }
}

impl From<scan_complete::Handler> for Handler {
    fn from(handler: scan_complete::Handler) -> Self {
        Self::ScanComplete(handler)
    }
}

impl From<touch_link_target::Handler> for Handler {
    fn from(handler: touch_link_target::Handler) -> Self {
        Self::TouchLinkTarget(handler)
    }
}
