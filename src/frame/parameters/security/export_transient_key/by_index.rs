//! Parameters for the [`Security::export_transient_key_by_index`](crate::Security::export_transient_key_by_index) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use silizium::Status;

use super::transient_key::TransientKey;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0112;

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
