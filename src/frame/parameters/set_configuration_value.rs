use crate::types::{EzspConfigId, EzspStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0053;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    config_id: EzspConfigId,
    value: u16,
}

impl Command {
    #[must_use]
    pub const fn new(config_id: EzspConfigId, value: u16) -> Self {
        Self { config_id, value }
    }

    #[must_use]
    pub const fn config_id(&self) -> EzspConfigId {
        self.config_id
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
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
