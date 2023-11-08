use crate::types::bool;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00E5;

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
    has_long_up_time: bool,
}

impl Response {
    #[must_use]
    pub const fn new(has_long_up_time: bool) -> Self {
        Self { has_long_up_time }
    }

    #[must_use]
    pub const fn has_long_up_time(&self) -> bool {
        self.has_long_up_time
    }
}
