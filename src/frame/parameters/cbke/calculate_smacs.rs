//! Parameters for the [`Cbke::calculate_smacs`](crate::Cbke::calculate_smacs) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{CertificateData, PublicKeyData, Status};

crate::frame::parameters::frame!(0x009F, { am_initiator: bool, partner_certificate: CertificateData, partner_ephemeral_public_key: PublicKeyData }, { status: u8 });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(
        am_initiator: bool,
        partner_certificate: CertificateData,
        partner_ephemeral_public_key: PublicKeyData,
    ) -> Self {
        Self {
            am_initiator,
            partner_certificate,
            partner_ephemeral_public_key,
        }
    }
}

/// Converts the response into `()` or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
