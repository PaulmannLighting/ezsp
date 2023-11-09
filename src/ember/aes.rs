use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct MmoHashContext {
    result: [u8; 16],
    length: u32,
}

impl MmoHashContext {
    #[must_use]
    pub const fn new(result: [u8; 16], length: u32) -> Self {
        Self { result, length }
    }

    #[must_use]
    pub const fn result(&self) -> &[u8; 16] {
        &self.result
    }

    pub const fn length(&self) -> u32 {
        self.length
    }
}
