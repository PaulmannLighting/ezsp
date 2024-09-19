use le_stream::derive::FromLeStream;

use crate::ember::node::Type;
use crate::ember::{Eui64, NodeId};
use crate::frame::Parameter;

const ID: u16 = 0x0023;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    index: u8,
    joining: bool,
    child_id: NodeId,
    child_eui64: Eui64,
    child_type: u8,
}

impl Handler {
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn joining(&self) -> bool {
        self.joining
    }

    #[must_use]
    pub const fn child_id(&self) -> NodeId {
        self.child_id
    }

    #[must_use]
    pub const fn child_eui64(&self) -> Eui64 {
        self.child_eui64
    }

    pub fn child_type(&self) -> Result<Type, u8> {
        Type::try_from(self.child_type)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
