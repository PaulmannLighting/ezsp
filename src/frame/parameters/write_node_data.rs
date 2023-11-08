use crate::types::{bool, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00FE;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    erase: bool,
}

impl Command {
    #[must_use]
    pub const fn new(erase: bool) -> Self {
        Self { erase }
    }

    #[must_use]
    pub const fn erase(&self) -> bool {
        self.erase
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
