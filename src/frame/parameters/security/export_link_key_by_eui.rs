use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManKey};
use siliconlabs::Status;

const ID: u16 = 0x0110;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.payload)
    }
}

/// Payload of the export link key by EUI64 command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    plaintext_key: ManKey,
    index: u8,
    key_data: ManApsKeyMetadata,
}

impl Payload {
    /// Returns the plain text key.
    #[must_use]
    pub const fn plaintext_key(&self) -> &ManKey {
        &self.plaintext_key
    }

    /// Returns the index.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    /// Returns the key metadata.
    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata {
        &self.key_data
    }
}
