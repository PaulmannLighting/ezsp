use crate::types::{EzspStatus, SecureEzspRandomNumber, SecureEzspSecurityLevel};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00CB;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    security_level: SecureEzspSecurityLevel,
    host_random_number: SecureEzspRandomNumber,
}

impl Command {
    #[must_use]
    pub const fn new(
        security_level: SecureEzspSecurityLevel,
        host_random_number: SecureEzspRandomNumber,
    ) -> Self {
        Self {
            security_level,
            host_random_number,
        }
    }

    #[must_use]
    pub const fn security_level(&self) -> SecureEzspSecurityLevel {
        self.security_level
    }

    #[must_use]
    pub const fn host_random_number(&self) -> SecureEzspRandomNumber {
        self.host_random_number
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
