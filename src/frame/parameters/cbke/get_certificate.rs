//! Parameters for the [`Cbke::get_certificate`](crate::Cbke::get_certificate) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{CertificateData, Status};

crate::frame::parameters::frame!(0x00A5, {}, { status: u8, local_cert: CertificateData });

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
