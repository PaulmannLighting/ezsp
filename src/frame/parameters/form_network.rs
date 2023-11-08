use crate::types::{EmberNetworkParameters, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x001E;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    parameters: EmberNetworkParameters,
}

impl Command {
    #[must_use]
    pub const fn new(parameters: EmberNetworkParameters) -> Self {
        Self { parameters }
    }

    #[must_use]
    pub const fn parameters(&self) -> EmberNetworkParameters {
        self.parameters
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
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
