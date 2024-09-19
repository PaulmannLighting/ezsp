use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::beacon::ClassificationParams;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00EF;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    param: ClassificationParams,
}

impl Command {
    #[must_use]
    pub const fn new(param: ClassificationParams) -> Self {
        Self { param }
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

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
