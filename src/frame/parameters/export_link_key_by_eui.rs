use crate::ember::types::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManKey};
use siliconlabs::Status;

pub const ID: u16 = 0x0110;

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
    plaintext_key: ManKey,
    index: u8,
    key_data: ManApsKeyMetadata,
    status: u32,
}

impl Response {
    #[must_use]
    pub const fn new(
        plaintext_key: ManKey,
        index: u8,
        key_data: ManApsKeyMetadata,
        status: Status,
    ) -> Self {
        Self {
            plaintext_key,
            index,
            key_data,
            status: status.into(),
        }
    }

    #[must_use]
    pub const fn plaintext_key(&self) -> &ManKey {
        &self.plaintext_key
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata {
        &self.key_data
    }

    pub fn status(&self) -> Result<Status, u32> {
        Status::try_from(self.status)
    }
}
