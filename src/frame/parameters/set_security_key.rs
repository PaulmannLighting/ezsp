use crate::ember::key::Data;
use crate::ezsp::{SecurityType, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00CA;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    key: Data,
    security_type: SecurityType,
}

impl Command {
    #[must_use]
    pub const fn new(key: Data, security_type: SecurityType) -> Self {
        Self { key, security_type }
    }

    #[must_use]
    pub const fn key(&self) -> Data {
        self.key
    }

    #[must_use]
    pub const fn security_type(&self) -> SecurityType {
        self.security_type
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
