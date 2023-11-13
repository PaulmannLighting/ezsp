use crate::ember::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManContext, ManFlags, ManKey};
use siliconlabs::Status;

pub const ID: u16 = 0x0111;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    context: ManContext,
    eui64: Eui64,
    plaintext_key: ManKey,
    flags: u8,
}

impl Command {
    #[must_use]
    pub fn new(context: ManContext, eui64: Eui64, plaintext_key: ManKey, flags: ManFlags) -> Self {
        Self {
            context,
            eui64,
            plaintext_key,
            flags: flags.into(),
        }
    }

    #[must_use]
    pub const fn context(&self) -> &ManContext {
        &self.context
    }

    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    #[must_use]
    pub const fn plaintext_key(&self) -> &ManKey {
        &self.plaintext_key
    }

    pub fn flags(&self) -> Result<ManFlags, u8> {
        ManFlags::try_from(self.flags)
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u32,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u32> {
        Status::try_from(self.status)
    }
}
