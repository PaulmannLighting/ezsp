use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManKey};
use siliconlabs::Status;

const ID: u16 = 0x010F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    eui: Eui64,
    plaintext_key: ManKey,
    key_data: ManApsKeyMetadata,
    status: u32,
}

impl Response {
    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
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

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = Self;

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve().map(|()| self)
    }
}
