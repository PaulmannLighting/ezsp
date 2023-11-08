use crate::types::EzspStatus;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00CC;

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
    status: EzspStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }
}
