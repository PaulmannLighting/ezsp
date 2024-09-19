use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::neighbor::TableEntry;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0079;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    value: TableEntry,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = TableEntry;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve().map(|()| self.value)
    }
}
