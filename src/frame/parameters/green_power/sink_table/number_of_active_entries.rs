use crate::frame::Parameter;
use crate::types::UintT;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0118;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    number_of_entries: UintT,
}

impl Response {
    #[must_use]
    pub const fn number_of_entries(&self) -> UintT {
        self.number_of_entries
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
