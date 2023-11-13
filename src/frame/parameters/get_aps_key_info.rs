use crate::ember::types::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext};
use siliconlabs::Status;

pub const ID: u16 = 0x010C;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    context_in: ManContext,
}

impl Command {
    #[must_use]
    pub const fn new(context_in: ManContext) -> Self {
        Self { context_in }
    }

    #[must_use]
    pub const fn context_in(&self) -> &ManContext {
        &self.context_in
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    eui: Eui64,
    key_data: ManApsKeyMetadata,
    status: u32,
}

impl Response {
    #[must_use]
    pub const fn new(eui: Eui64, key_data: ManApsKeyMetadata, status: Status) -> Self {
        Self {
            eui,
            key_data,
            status: status.into(),
        }
    }

    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
    }

    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata {
        &self.key_data
    }

    pub fn status(&self) -> Result<Status, u32> {
        Status::try_from(self.status)
    }
}
