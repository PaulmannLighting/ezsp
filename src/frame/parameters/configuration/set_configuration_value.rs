use crate::ezsp::config::Id;
use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0053;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    config_id: u8,
    value: u16,
}

impl Command {
    #[must_use]
    pub fn new(config_id: Id, value: u16) -> Self {
        Self {
            config_id: config_id.into(),
            value,
        }
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

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
