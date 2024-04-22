use crate::ember::PanId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0057;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    new_pan: PanId,
}

impl Command {
    #[must_use]
    pub const fn new(new_pan: PanId) -> Self {
        Self { new_pan }
    }

    #[must_use]
    pub const fn new_pan(&self) -> PanId {
        self.new_pan
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: bool,
}

impl Response {
    #[must_use]
    pub const fn new(status: bool) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> bool {
        self.status
    }
}
