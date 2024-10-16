use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00D1;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    routing_shortcut_thresh: u8,
}

impl Response {
    #[must_use]
    pub const fn routing_shortcut_thresh(&self) -> u8 {
        self.routing_shortcut_thresh
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
