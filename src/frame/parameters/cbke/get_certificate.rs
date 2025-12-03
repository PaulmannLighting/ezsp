//! Parameters for the [`Cbke::get_certificate`](crate::Cbke::get_certificate) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{CertificateData, Status};
use crate::frame::Parameter;

const ID: u16 = 0x00A5;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    local_cert: CertificateData,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into [`CertificateData`]
/// or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Response> for CertificateData {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.local_cert),
            other => Err(other.into()),
        }
    }
}
