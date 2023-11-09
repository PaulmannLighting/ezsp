use crate::types::EmberEntropySource;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x004F;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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
