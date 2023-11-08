use crate::types::EmberEUI64;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0062;

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
    sender_eui64: EmberEUI64,
}

impl Response {
    #[must_use]
    pub const fn new(sender_eui64: EmberEUI64) -> Self {
        Self { sender_eui64 }
    }

    #[must_use]
    pub const fn sender_eui64(&self) -> EmberEUI64 {
        self.sender_eui64
    }
}
