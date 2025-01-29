//! Transient key structure definitions.

use le_stream::derive::FromLeStream;
use silizium::zigbee::security::man::{ApsKeyMetadata, Context, Key};

/// The exported transient key.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct TransientKey {
    context: Context,
    plaintext_key: Key,
    key_data: ApsKeyMetadata,
}

impl TransientKey {
    /// Returns the context.
    #[must_use]
    pub const fn context(&self) -> &Context {
        &self.context
    }

    /// Returns the plaintext key.
    #[must_use]
    pub const fn plaintext_key(&self) -> &Key {
        &self.plaintext_key
    }

    /// Returns the key metadata.
    #[must_use]
    pub const fn key_data(&self) -> &ApsKeyMetadata {
        &self.key_data
    }
}
