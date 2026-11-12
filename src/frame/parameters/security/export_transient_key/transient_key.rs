//! Transient key structure definitions.

use std::fmt::{self, Debug, Formatter};

use le_stream::{FromLeStream, ToLeStream};
use silizium::zigbee::security::man::{ApsKeyMetadata, Context, Key};

/// The exported transient key.
#[derive(Clone, Eq, PartialEq, FromLeStream, ToLeStream)]
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

impl Debug for TransientKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransientKey")
            .field("context", &self.context)
            .field("key_data", &self.key_data)
            .field("plaintext_key", &"[REDACTED]")
            .finish()
    }
}
