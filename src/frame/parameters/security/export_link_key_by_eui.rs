use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManKey};
use siliconlabs::Status;

use crate::ember::Eui64;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x010D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    eui: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui: Eui64) -> Self {
        Self { eui }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    payload: Payload,
    status: u32,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for Payload {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(response.payload),
            other => Err(other.into()),
        }
    }
}

/// Payload of the export link key by EUI64 command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    plaintext_key: ManKey,
    index: u8,
    key_data: ManApsKeyMetadata<u16>,
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
    pub const fn key_data(&self) -> &ManApsKeyMetadata<u16> {
        &self.key_data
    }
}
