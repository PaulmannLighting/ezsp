use crate::ember::NodeId;
use crate::frame;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0106;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    child_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(child_index: u8) -> Self {
        Self { child_index }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    child_id: NodeId,
}

impl Response {
    #[must_use]
    pub const fn child_id(&self) -> NodeId {
        self.child_id
    }
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}
