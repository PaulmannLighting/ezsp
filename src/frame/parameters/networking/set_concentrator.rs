use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::concentrator::Parameters;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0010;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
