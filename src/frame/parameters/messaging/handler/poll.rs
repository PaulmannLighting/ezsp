use le_stream::derive::FromLeBytes;

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x0044;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    child_id: NodeId,
    transmit_expected: bool,
}

impl Handler {
    #[must_use]
    pub const fn child_id(&self) -> NodeId {
        self.child_id
    }

    #[must_use]
    pub const fn transmit_expected(&self) -> bool {
        self.transmit_expected
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
