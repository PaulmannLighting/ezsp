use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x009D;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    delay: u16,
}

impl Command {
    #[must_use]
    pub const fn new(delay: u16) -> Self {
        Self { delay }
    }

    #[must_use]
    pub const fn delay(&self) -> u16 {
        self.delay
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
