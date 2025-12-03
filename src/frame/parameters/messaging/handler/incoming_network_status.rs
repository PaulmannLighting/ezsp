use le_stream::FromLeStream;

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x00C4;

/// A callback invoked when a network status/route error message is received.
///
/// The error indicates that there was a problem sending/receiving messages from the target node.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    error_code: u8,
    target: NodeId,
}

impl Handler {
    /// One byte over-the-air error code from network status message.
    #[must_use]
    pub const fn error_code(&self) -> u8 {
        self.error_code
    }

    /// The short ID of the remote node.
    #[must_use]
    pub const fn target(&self) -> NodeId {
        self.target
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
