use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0049;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    value: u16,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = u16;

    fn resolve(self) -> crate::Result<Self::Output> {
        Status::try_from(self.status).resolve().map(|_| self.value)
    }
}
