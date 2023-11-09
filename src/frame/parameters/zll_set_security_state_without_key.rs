use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberZllInitialSecurityState,EmberStatus};

pub const ID: u16 = 0x00CF;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    security_state: EmberZllInitialSecurityState,
}

impl Command {
    #[must_use]
    pub const fn new(security_state: EmberZllInitialSecurityState) -> Self {
        Self { security_state }
    }

    #[must_use]
    pub const fn security_state(&self) -> EmberZllInitialSecurityState {
        self.security_state
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
