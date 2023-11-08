use crate::types::{bool, EmberEUI64};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x007E;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    remote_eui64: EmberEUI64,
    extended_timeout: bool,
}

impl Command {
    #[must_use]
    pub const fn new(remote_eui64: EmberEUI64, extended_timeout: bool) -> Self {
        Self {
            remote_eui64,
            extended_timeout,
        }
    }

    #[must_use]
    pub const fn remote_eui64(&self) -> EmberEUI64 {
        self.remote_eui64
    }

    #[must_use]
    pub const fn extended_timeout(&self) -> bool {
        self.extended_timeout
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
