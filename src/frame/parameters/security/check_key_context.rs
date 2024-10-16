use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::zigbee::security::ManContext;
use siliconlabs::Status;

use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0110;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
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
