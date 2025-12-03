//! Parameters for the [`Security::export_link_key_by_index`](crate::Security::export_link_key_by_index) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::{ApsKeyMetadata, Key};

use crate::Error;
use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x010F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
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

/// Payload of the export link key by index command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    eui: Eui64,
    plaintext_key: Key,
    key_data: ApsKeyMetadata,
}

impl Payload {
    /// Returns the EUI64.
    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
    }

    /// Returns the plaintext key.
    #[must_use]
    pub const fn plaintext_key(&self) -> &Key {
        &self.plaintext_key
    }

    /// Returns the key data.
    #[must_use]
    pub const fn key_data(&self) -> &ApsKeyMetadata {
        &self.key_data
    }
}
