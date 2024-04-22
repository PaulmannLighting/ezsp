use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::frame::Parameter;

const ID: u16 = 0x003C;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
