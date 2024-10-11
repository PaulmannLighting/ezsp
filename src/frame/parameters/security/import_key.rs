use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::zigbee::security::{ManContext, ManKey};
use siliconlabs::Status;

const ID: u16 = 0x0115;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    context: ManContext,
    key: ManKey,
}

impl Command {
    #[must_use]
    pub const fn new(context: ManContext, key: ManKey) -> Self {
        Self { context, key }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u32,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve().map(drop)
    }
}
