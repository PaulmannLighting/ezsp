use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberInitialSecurityState,EmberStatus};

pub const ID: u16 = 0x0068;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    state: EmberInitialSecurityState,
}

impl Command {
    #[must_use]
    pub const fn new(state: EmberInitialSecurityState) -> Self {
        Self { state }
    }

    #[must_use]
    pub const fn state(&self) -> EmberInitialSecurityState {
        self.state
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    success: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(success: EmberStatus) -> Self {
        Self { success }
    }

    #[must_use]
    pub const fn success(&self) -> EmberStatus {
        self.success
    }
}
