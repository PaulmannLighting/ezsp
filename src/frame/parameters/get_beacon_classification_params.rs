use crate::ember::beacon::ClassificationParams;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00F3;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    param: ClassificationParams,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, param: ClassificationParams) -> Self {
        Self {
            status: status.into(),
            param,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn param(&self) -> &ClassificationParams {
        &self.param
    }
}
