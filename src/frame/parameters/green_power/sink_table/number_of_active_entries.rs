use crate::frame::Identified;
use crate::types::UintT;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0118;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    number_of_entries: UintT,
}

impl Response {
    #[must_use]
    pub const fn number_of_entries(&self) -> UintT {
        self.number_of_entries
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
