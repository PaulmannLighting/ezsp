use le_stream::FromLeStream;

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x0044;

/// Indicates that the local node received a data poll from a child.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    child_id: NodeId,
    transmit_expected: bool,
}

impl Handler {
    /// The node ID of the child that is requesting data.
    #[must_use]
    pub const fn child_id(&self) -> NodeId {
        self.child_id
    }

    /// True if transmit is expected, false otherwise.
    #[must_use]
    pub const fn transmit_expected(&self) -> bool {
        self.transmit_expected
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
