use crate::types::{EmberCurrentSecurityState, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0069;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    state: EmberCurrentSecurityState,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, state: EmberCurrentSecurityState) -> Self {
        Self { status, state }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn state(&self) -> EmberCurrentSecurityState {
        self.state
    }
}
