use crate::ezsp::{SecurityType, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00CD;

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
    return_security_type: SecurityType,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, return_security_type: SecurityType) -> Self {
        Self {
            status: status.into(),
            return_security_type,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn return_security_type(&self) -> SecurityType {
        self.return_security_type
    }
}
