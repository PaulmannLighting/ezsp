use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext, ManKey};
use siliconlabs::Status;

const ID: u16 = 0x0112;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    context: ManContext,
    plaintext_key: ManKey,
    key_data: ManApsKeyMetadata,
    status: u32,
}

impl Response {
    #[must_use]
    pub fn new(
        context: ManContext,
        plaintext_key: ManKey,
        key_data: ManApsKeyMetadata,
        status: Status,
    ) -> Self {
        Self {
            context,
            plaintext_key,
            key_data,
            status: status.into(),
        }
    }

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

    pub fn status(&self) -> Result<Status, u32> {
        Status::try_from(self.status)
    }
}
