use crate::ember::PanId;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0057;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    new_pan: PanId,
}

impl Command {
    #[must_use]
    pub const fn new(new_pan: PanId) -> Self {
        Self { new_pan }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: bool,
}

impl Response {
    #[must_use]
    pub const fn status(&self) -> bool {
        self.status
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
