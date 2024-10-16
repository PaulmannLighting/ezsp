use le_stream::derive::FromLeStream;

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x007C;

/// A callback invoked by the EmberZNet stack when an id conflict is discovered, that is,
/// two different nodes in the network were found to be using the same short id.
///
/// The stack automatically removes the conflicting short id from its internal tables
/// (address, binding, route, neighbor, and child tables).
///
/// The application should discontinue any other use of the id.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    id: NodeId,
}

impl Handler {
    /// The short id for which a conflict was detected.
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
