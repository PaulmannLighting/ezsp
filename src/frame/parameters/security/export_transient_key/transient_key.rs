//! Transient key structure definitions.

use le_stream::derive::FromLeStream;
use siliconlabs::zigbee::security::man::{ApsKeyMetadata, Context, Key};

use crate::ember::Eui64;

/// The exported transient key.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct TransientKey {
    context: Context<Eui64>,
    plaintext_key: Key,
    key_data: ApsKeyMetadata<u16>,
}

impl TransientKey {
    /// Returns the context.
    #[must_use]
    pub const fn context(&self) -> &Context<Eui64> {
        &self.context
    }

    /// Returns the plaintext key.
    #[must_use]
    pub const fn plaintext_key(&self) -> &Key {
        &self.plaintext_key
    }

    /// Returns the key metadata.
    #[must_use]
    pub const fn key_data(&self) -> &ApsKeyMetadata<u16> {
        &self.key_data
    }
}
