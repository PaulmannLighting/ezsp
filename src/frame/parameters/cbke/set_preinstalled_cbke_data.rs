//! Parameters for the [`Cbke::set_preinstalled_cbke_data`](crate::Cbke::set_preinstalled_cbke_data) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{CertificateData, PrivateKeyData, PublicKeyData, Status};

crate::frame::parameters::frame!(0x00A2, { ca_public: PublicKeyData, my_cert: CertificateData, my_key: PrivateKeyData }, { status: u8 });

impl Command {
    #[must_use]
    pub const fn new(
        ca_public: PublicKeyData,
        my_cert: CertificateData,
        my_key: PrivateKeyData,
    ) -> Self {
        Self {
            ca_public,
            my_cert,
            my_key,
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
