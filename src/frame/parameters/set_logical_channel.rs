use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00BA;

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
    logical_channel: u8,
}

impl Response {
    #[must_use]
    pub const fn new(logical_channel: u8) -> Self {
        Self { logical_channel }
    }

    #[must_use]
    pub const fn logical_channel(&self) -> u8 {
        self.logical_channel
    }
}
