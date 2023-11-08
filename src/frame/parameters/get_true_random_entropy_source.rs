use crate::types::EmberEntropySource;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x004F;

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
    entropy_source: EmberEntropySource,
}

impl Response {
    #[must_use]
    pub const fn new(entropy_source: EmberEntropySource) -> Self {
        Self { entropy_source }
    }

    #[must_use]
    pub const fn entropy_source(&self) -> EmberEntropySource {
        self.entropy_source
    }
}
