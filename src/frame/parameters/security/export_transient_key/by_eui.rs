//! Parameters for the [`Security::export_transient_key_by_eui`](crate::Security::export_transient_key_by_eui) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use silizium::Status;

use super::transient_key::TransientKey;
use crate::Error;
use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x0113;

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
    payload: TransientKey,
    status: u32,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into [`TransientKey`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for TransientKey {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(response.payload),
            other => Err(other.into()),
        }
    }
}
