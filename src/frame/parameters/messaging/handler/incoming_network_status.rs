use le_stream::derive::FromLeStream;

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x00C4;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    error_code: u8,
    target: NodeId,
}

impl Handler {
    #[must_use]
    pub const fn error_code(&self) -> u8 {
        self.error_code
    }

    #[must_use]
    pub const fn target(&self) -> NodeId {
        self.target
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Handler {
    const ID: u16 = ID;
}
