//! Parameters for the [`Cbke::get_certificate283k1`](crate::Cbke::get_certificate283k1) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Certificate283k1Data, Status};

crate::frame::parameters::frame!(0x00EC, {}, { status: u8, local_cert: Certificate283k1Data });

/// Converts the response into [`Certificate283k1Data`]
/// or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Response> for Certificate283k1Data {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.local_cert),
            other => Err(other.into()),
        }
    }
}
