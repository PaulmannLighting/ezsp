use crate::types::{EmberDutyCycleState, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0035;

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
    returned_state: EmberDutyCycleState,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, returned_state: EmberDutyCycleState) -> Self {
        Self {
            status,
            returned_state,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn returned_state(&self) -> EmberDutyCycleState {
        self.returned_state
    }
}
