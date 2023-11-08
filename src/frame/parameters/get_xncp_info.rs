use crate::types::EmberStatus;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0013;

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
    manufacturer_id: u16,
    version_number: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, manufacturer_id: u16, version_number: u16) -> Self {
        Self {
            status,
            manufacturer_id,
            version_number,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn manufacturer_id(&self) -> u16 {
        self.manufacturer_id
    }

    #[must_use]
    pub const fn version_number(&self) -> u16 {
        self.version_number
    }
}
