use le_stream::derive::FromLeStream;

use crate::ember::{Eui64, NodeId};
use crate::frame;
use crate::frame::Parameter;

const ID: u16 = 0x007D;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    source: NodeId,
    long_id: Eui64,
    cost: u8,
}

impl Handler {
    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    #[must_use]
    pub const fn long_id(&self) -> Eui64 {
        self.long_id
    }

    #[must_use]
    pub const fn cost(&self) -> u8 {
        self.cost
    }
}

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
