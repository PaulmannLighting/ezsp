use le_stream::derive::FromLeBytes;

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x007C;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    id: NodeId,
}

impl Handler {
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
