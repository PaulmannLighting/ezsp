//! Parameters for the [`Cbke::calculate_smacs283k1`](crate::Cbke::calculate_smacs283k1) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Certificate283k1Data, PublicKey283k1Data, Status};

crate::frame::parameters::frame!(0x00EA, { am_initiator: bool, partner_certificate: Certificate283k1Data, partner_ephemeral_public_key: PublicKey283k1Data }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(
        am_initiator: bool,
        partner_certificate: Certificate283k1Data,
        partner_ephemeral_public_key: PublicKey283k1Data,
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
