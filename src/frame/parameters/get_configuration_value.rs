use crate::types::{EzspConfigId, EzspStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0052;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    config_id: EzspConfigId,
}

impl Command {
    #[must_use]
    pub const fn new(config_id: EzspConfigId) -> Self {
        Self { config_id }
    }

    #[must_use]
    pub const fn config_id(&self) -> EzspConfigId {
        self.config_id
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EzspStatus,
    value: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus, value: u16) -> Self {
        Self { status, value }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }
}
