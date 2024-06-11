use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::frame::Parameter;

const ID: u16 = 0x0016;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    descriptor: u16,
}

impl Command {
    #[must_use]
    pub const fn new(descriptor: u16) -> Self {
        Self { descriptor }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
