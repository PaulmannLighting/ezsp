use le_stream::derive::{FromLeStream, ToLeStream};

/// The hash context for an ongoing hash operation.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct MmoHashContext {
    result: [u8; 16],
    length: u32,
}

impl MmoHashContext {
    /// Create a new hash context.
    #[must_use]
    pub const fn new(result: [u8; 16], length: u32) -> Self {
        Self { result, length }
    }

    /// Return the result of ongoing the hash operation.
    #[must_use]
    pub const fn result(&self) -> &[u8; 16] {
        &self.result
    }

    /// Return the total length of the data that has been hashed so far.
    #[must_use]
    pub const fn length(&self) -> u32 {
        self.length
    }
}
