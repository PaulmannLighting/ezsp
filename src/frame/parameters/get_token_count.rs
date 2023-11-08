use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0100;

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
    count: u8,
}

impl Response {
    #[must_use]
    pub const fn new(count: u8) -> Self {
        Self { count }
    }

    #[must_use]
    pub const fn count(&self) -> u8 {
        self.count
    }
}
