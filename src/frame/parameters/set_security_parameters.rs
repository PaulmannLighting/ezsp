use crate::ezsp::{SecureRandomNumber, SecurityLevel, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00CB;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    security_level: SecurityLevel,
    host_random_number: SecureRandomNumber,
}

impl Command {
    #[must_use]
    pub const fn new(
        security_level: SecurityLevel,
        host_random_number: SecureRandomNumber,
    ) -> Self {
        Self {
            security_level,
            host_random_number,
        }
    }

    #[must_use]
    pub const fn security_level(&self) -> SecurityLevel {
        self.security_level
    }

    #[must_use]
    pub const fn host_random_number(&self) -> &SecureRandomNumber {
        &self.host_random_number
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
