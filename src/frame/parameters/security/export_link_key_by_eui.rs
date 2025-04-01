//! Parameters for the [`Security::export_link_key_by_eui`](crate::Security::export_link_key_by_eui) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::{ApsKeyMetadata, Key};

use crate::Error;
use crate::ember::Eui64;
use crate::frame::Parameter;

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

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    payload: Payload,
    status: u32,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into [`Payload`] or an appropriate [`Error`] depending on its status.
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
    plaintext_key: Key,
    index: u8,
    key_data: ApsKeyMetadata,
}

impl Payload {
    /// Returns the plain text key.
    #[must_use]
    pub const fn plaintext_key(&self) -> &Key {
        &self.plaintext_key
    }

    /// Returns the index.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    /// Returns the key metadata.
    #[must_use]
    pub const fn key_data(&self) -> &ApsKeyMetadata {
        &self.key_data
    }
}
