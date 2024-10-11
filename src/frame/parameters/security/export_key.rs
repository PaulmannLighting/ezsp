use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::zigbee::security::{ManContext, ManKey};
use siliconlabs::Status;

const ID: u16 = 0x0114;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    context: ManContext,
}

impl Command {
    #[must_use]
    pub const fn new(context: ManContext) -> Self {
        Self { context }
    }
}

impl Parameter<u16> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    key: ManKey,
    status: u32,
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ManKey;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve().map(|_| self.key)
    }
}
