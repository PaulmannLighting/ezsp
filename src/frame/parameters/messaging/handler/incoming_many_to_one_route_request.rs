use le_stream::derive::FromLeStream;

use crate::ember::{Eui64, NodeId};
use crate::frame::Identified;

const ID: u16 = 0x007D;

/// A callback indicating that a many-to-one route to the concentrator
/// with the given short and long id is available for use.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    source: NodeId,
    long_id: Eui64,
    cost: u8,
}

impl Handler {
    /// The short id of the concentrator.
    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    /// The EUI64 of the concentrator.
    #[must_use]
    pub const fn long_id(&self) -> Eui64 {
        self.long_id
    }

    /// The path cost to the concentrator.
    ///
    /// The cost may decrease as additional route request packets for this discovery arrive,
    /// but the callback is made only once.
    #[must_use]
    pub const fn cost(&self) -> u8 {
        self.cost
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
