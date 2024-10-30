//! Parameters for the [`Security::get_aps_key_info`](crate::Security::get_aps_key_info) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::{ManApsKeyMetadata, ManContext};
use siliconlabs::Status;

use crate::ember::Eui64;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x010C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context_in: ManContext<Eui64>,
}

impl Command {
    #[must_use]
    pub const fn new(context_in: ManContext<Eui64>) -> Self {
        Self { context_in }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    payload: KeyInfo,
    status: u32,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into [`KeyInfo`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for KeyInfo {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(response.payload),
            other => Err(other.into()),
        }
    }
}

/// The retrieved key information.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct KeyInfo {
    eui: Eui64,
    key_data: ManApsKeyMetadata<u16>,
}

impl KeyInfo {
    /// Returns the EUI64.
    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
    }

    /// Returns the key data.
    #[must_use]
    pub const fn key_data(&self) -> &ManApsKeyMetadata<u16> {
        &self.key_data
    }
}
