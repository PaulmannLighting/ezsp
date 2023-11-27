use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct SecurityToken {
    bitmask: u32,
    key_index: u8,
    encryption_key: [u8; 16],
    preconfigured_key: [u8; 16],
}

impl SecurityToken {
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

    #[must_use]
    pub const fn bitmask(&self) -> u32 {
        self.bitmask
    }

    #[must_use]
    pub const fn key_index(&self) -> u8 {
        self.key_index
    }

    #[must_use]
    pub const fn encryption_key(&self) -> &[u8; 16] {
        &self.encryption_key
    }

    #[must_use]
    pub const fn preconfigured_key(&self) -> &[u8; 16] {
        &self.preconfigured_key
    }
}
