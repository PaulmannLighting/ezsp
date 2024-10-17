use le_stream::derive::FromLeStream;
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext, ManKey};

use crate::ember::Eui64;

/// The exported transient key.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct TransientKey {
    context: ManContext<Eui64>,
    plaintext_key: ManKey,
    key_data: ManApsKeyMetadata<u16>,
}

impl TransientKey {
    /// Returns the context.
    #[must_use]
    pub const fn context(&self) -> &ManContext<Eui64> {
        &self.context
    }

    /// Returns the plaintext key.
    #[must_use]
    pub const fn plaintext_key(&self) -> &ManKey {
        &self.plaintext_key
    }

    /// Returns the key metadata.
    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata<u16> {
        &self.key_data
    }
}
