use le_stream::derive::{FromLeStream, ToLeStream};

/// Public API for ZLL stack security token.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct SecurityToken {
    bitmask: u32,
    key_index: u8,
    encryption_key: [u8; 16],
    preconfigured_key: [u8; 16],
}

impl SecurityToken {
    /// Create a new ZLL stack security token.
    #[must_use]
    pub const fn new(
        bitmask: u32,
        key_index: u8,
        encryption_key: [u8; 16],
        preconfigured_key: [u8; 16],
    ) -> Self {
        Self {
            bitmask,
            key_index,
            encryption_key,
            preconfigured_key,
        }
    }

    /// Return the token bitmask.
    #[must_use]
    pub const fn bitmask(&self) -> u32 {
        self.bitmask
    }

    /// Return the key index.
    #[must_use]
    pub const fn key_index(&self) -> u8 {
        self.key_index
    }

    /// Return the encryption key.
    #[must_use]
    pub const fn encryption_key(&self) -> &[u8; 16] {
        &self.encryption_key
    }

    /// Return the preconfigured key.
    #[must_use]
    pub const fn preconfigured_key(&self) -> &[u8; 16] {
        &self.preconfigured_key
    }
}
