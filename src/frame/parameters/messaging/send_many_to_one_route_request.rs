use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};

const ID: u16 = 0x0041;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    concentrator_type: u16,
    radius: u8,
}

impl Command {
    #[must_use]
    pub const fn new(concentrator_type: u16, radius: u8) -> Self {
        Self {
            concentrator_type,
            radius,
        }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
