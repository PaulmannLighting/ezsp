use le_stream::derive::FromLeStream;

use crate::ember::NodeId;
use crate::frame;
use crate::frame::Parameter;

const ID: u16 = 0x007C;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    id: NodeId,
}

impl Handler {
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }
}

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
