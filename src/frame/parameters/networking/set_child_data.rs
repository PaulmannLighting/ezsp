use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{child, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00AC;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    index: u8,
    child_data: child::Data,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, child_data: child::Data) -> Self {
        Self { index, child_data }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
