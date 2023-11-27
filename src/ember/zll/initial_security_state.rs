use crate::ember::key::Data;
use crate::ember::zll::KeyIndex;
use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct InitialSecurityState {
    bitmask: u32,
    key_index: u8,
    encryption_key: Data,
    preconfigured_key: Data,
}

impl InitialSecurityState {
    #[must_use]
    pub fn new(
        bitmask: u32,
        key_index: KeyIndex,
        encryption_key: Data,
        preconfigured_key: Data,
    ) -> Self {
        Self {
            bitmask,
            key_index: key_index.into(),
            encryption_key,
            preconfigured_key,
        }
    }

    #[must_use]
    pub const fn bitmask(&self) -> u32 {
        self.bitmask
    }

    pub fn key_index(&self) -> Result<KeyIndex, u8> {
        KeyIndex::try_from(self.key_index)
    }

    #[must_use]
    pub const fn encryption_key(&self) -> &Data {
        &self.encryption_key
    }

    #[must_use]
    pub const fn preconfigured_key(&self) -> &Data {
        &self.preconfigured_key
    }
}
