use crate::ember::Eui64;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext, ManKey};
use siliconlabs::Status;

const ID: u16 = 0x0113;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    eui: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui: Eui64) -> Self {
        Self { eui }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

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

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    payload: Payload,
    status: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = Payload;

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.payload)
    }
}
