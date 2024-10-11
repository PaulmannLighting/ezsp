use le_stream::derive::FromLeStream;

use crate::ember::{NodeId, Status};
use crate::frame::Parameter;

const ID: u16 = 0x0080;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    target: NodeId,
}

impl Handler {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn target(&self) -> NodeId {
        self.target
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Handler {
    const ID: u16 = ID;
}
