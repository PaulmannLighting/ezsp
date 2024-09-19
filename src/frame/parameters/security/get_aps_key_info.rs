use crate::ember::Eui64;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext};
use siliconlabs::Status;

const ID: u16 = 0x010C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    context_in: ManContext,
}

impl Command {
    #[must_use]
    pub const fn new(context_in: ManContext) -> Self {
        Self { context_in }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    eui: Eui64,
    key_data: ManApsKeyMetadata,
    status: u32,
}

impl Response {
    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
    }

    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata {
        &self.key_data
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = Self;

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve().map(|_| self)
    }
}
