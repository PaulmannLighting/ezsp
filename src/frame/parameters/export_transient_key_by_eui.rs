use crate::ember::types::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext, ManKey};
use siliconlabs::Status;

pub const ID: u16 = 0x0113;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    eui: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui: Eui64) -> Self {
        Self { eui }
    }

    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    context: ManContext,
    plaintext_key: ManKey,
    key_data: ManApsKeyMetadata,
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(
        context: ManContext,
        plaintext_key: ManKey,
        key_data: ManApsKeyMetadata,
        status: Status,
    ) -> Self {
        Self {
            context,
            plaintext_key,
            key_data,
            status,
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

    #[must_use]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
