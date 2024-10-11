use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::concentrator::Parameters;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0010;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    on: bool,
    parameters: Parameters,
}

impl From<Option<Parameters>> for Command {
    fn from(parameters: Option<Parameters>) -> Self {
        Self {
            on: parameters.is_some(),
            parameters: parameters.unwrap_or_default(),
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
