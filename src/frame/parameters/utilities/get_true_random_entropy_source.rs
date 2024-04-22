use crate::ember::entropy::Source;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x004F;

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
    entropy_source: u8,
}

impl Response {
    #[must_use]
    pub fn new(entropy_source: Source) -> Self {
        Self {
            entropy_source: entropy_source.into(),
        }
    }

    pub fn entropy_source(&self) -> Result<Source, u8> {
        Source::try_from(self.entropy_source)
    }
}
