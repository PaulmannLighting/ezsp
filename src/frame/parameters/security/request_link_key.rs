use crate::ember::{Eui64, Status};
use crate::frame::Parameter;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0014;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    partner: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(partner: Eui64) -> Self {
        Self { partner }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
