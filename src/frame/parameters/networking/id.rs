use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::frame;
use crate::frame::Parameter;

const ID: u16 = 0x0107;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    child_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(child_id: NodeId) -> Self {
        Self { child_id }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    child_index: u8,
}

impl Response {
    #[must_use]
    pub const fn child_index(&self) -> u8 {
        self.child_index
    }
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}
