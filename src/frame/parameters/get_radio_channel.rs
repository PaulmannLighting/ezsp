use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00FF;

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
    channel: u8,
}

impl Response {
    #[must_use]
    pub const fn new(channel: u8) -> Self {
        Self { channel }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}
