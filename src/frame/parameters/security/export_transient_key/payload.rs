use le_stream::derive::FromLeBytes;
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext, ManKey};

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Payload {
    context: ManContext,
    plaintext_key: ManKey,
    key_data: ManApsKeyMetadata,
}

impl Payload {
    #[must_use]
    pub const fn context(&self) -> &ManContext {
        &self.context
    }

    #[must_use]
    pub const fn plaintext_key(&self) -> &ManKey {
        &self.plaintext_key
    }

    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata {
        &self.key_data
    }
}
