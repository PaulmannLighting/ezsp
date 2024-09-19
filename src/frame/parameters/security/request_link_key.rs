use crate::ember::{Eui64, Status};
use crate::frame::Parameter;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0014;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    partner: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(partner: Eui64) -> Self {
        Self { partner }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
