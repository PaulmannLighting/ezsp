use crate::ember::beacon::ClassificationParams;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

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

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
