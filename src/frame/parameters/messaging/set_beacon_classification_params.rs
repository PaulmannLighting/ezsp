use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::beacon::ClassificationParams;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};

const ID: u16 = 0x00EF;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    param: ClassificationParams,
}

impl Command {
    #[must_use]
    pub const fn new(param: ClassificationParams) -> Self {
        Self { param }
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
