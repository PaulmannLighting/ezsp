//! Ember token types.

use le_stream::derive::{FromLeStream, ToLeStream};

/// Ember token data.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Data {
    size: u32,
    data: [u8; 64],
}

impl Data {
    /// Create new token data.
    #[must_use]
    pub const fn new(size: u32, data: [u8; 64]) -> Self {
        Self { size, data }
    }

    /// Return token data size in bytes.
    #[must_use]
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Return token data array.
    #[must_use]
    pub const fn data(&self) -> &[u8; 64] {
        &self.data
    }
}

/// Information of a token in the token table.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Info {
    nvm3_key: u32,
    is_cnt: bool,
    is_idx: bool,
    size: u8,
    array_size: u8,
}

impl Info {
    /// Create new token info.
    #[must_use]
    pub const fn new(nvm3_key: u32, is_cnt: bool, is_idx: bool, size: u8, array_size: u8) -> Self {
        Self {
            nvm3_key,
            is_cnt,
            is_idx,
            size,
            array_size,
        }
    }

    /// Return NVM3 key of the token.
    #[must_use]
    pub const fn nvm3_key(&self) -> u32 {
        self.nvm3_key
    }

    /// Return whether token is a counter type.
    #[must_use]
    pub const fn is_cnt(&self) -> bool {
        self.is_cnt
    }

    /// Return whether token is an indexed token.
    #[must_use]
    pub const fn is_idx(&self) -> bool {
        self.is_idx
    }

    /// Return size of the token.
    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    /// Return array size of the token.
    #[must_use]
    pub const fn array_size(&self) -> u8 {
        self.array_size
    }
}
