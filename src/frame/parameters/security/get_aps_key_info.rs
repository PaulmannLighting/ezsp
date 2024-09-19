use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext};
use siliconlabs::Status;

const ID: u16 = 0x010C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    eui: Eui64,
    key_data: ManApsKeyMetadata,
}

impl Payload {
    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
    }

    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata {
        &self.key_data
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    payload: Payload,
    status: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = Payload;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.payload)
    }
}
