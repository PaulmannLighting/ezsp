use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManContext, ManKey};
use siliconlabs::Status;

pub const ID: u16 = 0x0114;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    context: ManContext,
}

impl Command {
    #[must_use]
    pub const fn new(context: ManContext) -> Self {
        Self { context }
    }

    #[must_use]
    pub const fn context(&self) -> &ManContext {
        &self.context
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    key: ManKey,
    status: u32,
}

impl Response {
    #[must_use]
    pub const fn new(key: ManKey, status: Status) -> Self {
        Self {
            key,
            status: status.into(),
        }
    }

    #[must_use]
    pub const fn key(&self) -> &ManKey {
        &self.key
    }

    pub fn status(&self) -> Result<Status, u32> {
        Status::try_from(self.status)
    }
}
