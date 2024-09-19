use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::ManContext;
use siliconlabs::Status;

use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0110;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    context: ManContext,
}

impl Command {
    #[must_use]
    pub const fn new(context: ManContext) -> Self {
        Self { context }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u32,
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
