use crate::types::bool;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00E4;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    connected: bool,
}

impl Command {
    #[must_use]
    pub const fn new(connected: bool) -> Self {
        Self { connected }
    }

    #[must_use]
    pub const fn connected(&self) -> bool {
        self.connected
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
